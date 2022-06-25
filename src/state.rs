use std::ops::DerefMut;
use std::time::Duration;

use bevy_asset::{Assets, Handle};
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_sprite::prelude::*;

use crate::{animation::Mode, Play, PlaySpeedMultiplier, SpriteSheetAnimation};

pub(crate) fn maintenance_systems() -> SystemSet {
    SystemSet::new().with_system(insert).with_system(remove)
}

pub(crate) fn post_update_systems() -> SystemSet {
    SystemSet::new().with_system(animate)
}

/// Animation state component which is automatically inserted/removed
///
/// It can be used to reset the animation state.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use benimator::SpriteSheetAnimationState;
///
/// fn restart_anim_from_start(mut query: Query<&mut SpriteSheetAnimationState>) {
///   for mut state in query.iter_mut() {
///     state.reset();
///   }
/// }
/// ```
#[derive(Default, Component)]
pub struct SpriteSheetAnimationState {
    current_frame: usize,
    elapsed_in_frame: Duration,
    // Control ping_pong backward frame navigation.
    going_backward: bool,
}

impl SpriteSheetAnimationState {
    /// Reset animation state
    ///
    /// The animation will restart from the first frame, like if the animation was freshly spawned.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns the index of the current frame
    ///
    /// The index is relative to the animation sequence. **not** to the sprite-sheet.
    #[must_use]
    pub fn current_frame_index(&self) -> usize {
        self.current_frame
    }

    /// Update the animation and the sprite (if necessary)
    ///
    /// Returns true if the animation has ended
    fn update(
        &mut self,
        mut sprite: impl DerefMut<Target = TextureAtlasSprite>,
        animation: &SpriteSheetAnimation,
        delta: Duration,
    ) -> bool {
        debug_assert!(animation.has_frames());

        let mut frame = animation.frames[self.current_frame % animation.frames.len()];
        sprite.index = frame.index;

        self.elapsed_in_frame += delta;
        while self.elapsed_in_frame >= frame.duration {
            match animation.mode {
                Mode::RepeatFrom(loop_from) => {
                    if self.current_frame < animation.frames.len() - 1 {
                        self.current_frame += 1;
                    } else {
                        self.current_frame = loop_from;
                    }
                }
                Mode::PingPong => {
                    if self.going_backward {
                        if self.current_frame == 0 {
                            self.going_backward = false;
                            self.current_frame += 1;
                        } else {
                            self.current_frame -= 1;
                        }
                    } else if self.current_frame < animation.frames.len() - 1 {
                        self.current_frame += 1;
                    } else {
                        self.going_backward = true;
                        self.current_frame -= 1;
                    }
                }
                Mode::Once => {
                    if self.current_frame < animation.frames.len() - 1 {
                        self.current_frame += 1;
                    } else {
                        self.reset();
                        return true;
                    }
                }
            }

            self.elapsed_in_frame -= frame.duration;
            frame = animation.frames[self.current_frame];
            sprite.index = frame.index;
        }

        false
    }
}

fn insert(
    mut commands: Commands<'_, '_>,
    query: Query<
        '_,
        '_,
        Entity,
        (
            With<Handle<SpriteSheetAnimation>>,
            Without<SpriteSheetAnimationState>,
        ),
    >,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(SpriteSheetAnimationState::default());
    }
}

fn remove(
    mut commands: Commands<'_, '_>,
    removed: RemovedComponents<'_, Handle<SpriteSheetAnimation>>,
) {
    for entity in removed.iter() {
        commands
            .entity(entity)
            .remove::<SpriteSheetAnimationState>();
    }
}

type AnimationSystemQuery<'a> = (
    Entity,
    &'a mut TextureAtlasSprite,
    &'a Handle<SpriteSheetAnimation>,
    &'a mut SpriteSheetAnimationState,
    Option<&'a PlaySpeedMultiplier>,
);

fn animate(
    mut commands: Commands<'_, '_>,
    time: Res<'_, Time>,
    animation_defs: Res<'_, Assets<SpriteSheetAnimation>>,
    mut animations: Query<'_, '_, AnimationSystemQuery<'_>, With<Play>>,
) {
    for (entity, sprite, animation, mut state, speed_multiplier) in
        animations.iter_mut().filter_map(
            |(entity, sprite, anim_handle, state, optional_speed_multiplier)| {
                animation_defs
                    .get(anim_handle)
                    .filter(|anim| anim.has_frames())
                    .map(|anim| (entity, sprite, anim, state, optional_speed_multiplier))
            },
        )
    {
        let delta = speed_multiplier
            .copied()
            .unwrap_or_default()
            .transform(time.delta());

        if state.update(sprite, animation, delta) {
            commands.entity(entity).remove::<Play>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[fixture]
    fn sprite() -> TextureAtlasSprite {
        TextureAtlasSprite::new(0)
    }

    #[fixture]
    fn sprite_at_second_frame() -> TextureAtlasSprite {
        TextureAtlasSprite::new(1)
    }

    #[fixture]
    fn sprite_at_third_frame() -> TextureAtlasSprite {
        TextureAtlasSprite::new(2)
    }

    #[fixture]
    fn frame_duration() -> Duration {
        Duration::from_secs(1)
    }

    #[fixture]
    fn smaller_duration(frame_duration: Duration) -> Duration {
        frame_duration - Duration::from_millis(1)
    }

    mod reset {
        use super::*;

        #[fixture]
        fn state() -> SpriteSheetAnimationState {
            SpriteSheetAnimationState {
                current_frame: 1,
                elapsed_in_frame: Duration::from_secs(1),
                going_backward: false,
            }
        }

        #[rstest]
        fn resets_current_frame(mut state: SpriteSheetAnimationState) {
            state.reset();
            assert_eq!(state.current_frame, 0);
        }

        #[rstest]
        fn resets_elapsed_time(mut state: SpriteSheetAnimationState) {
            state.reset();
            assert_eq!(state.elapsed_in_frame, Duration::ZERO);
        }
    }

    mod on_first_frame {
        use super::*;

        #[fixture]
        fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
            SpriteSheetAnimation::from_range(1..=3, frame_duration)
        }

        #[fixture]
        fn state() -> SpriteSheetAnimationState {
            SpriteSheetAnimationState::default()
        }

        #[rstest]
        fn nothing_happens_if_not_enough_time_has_elapsed_and_index_is_already_set(
            mut state: SpriteSheetAnimationState,
            mut sprite_at_second_frame: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            smaller_duration: Duration,
        ) {
            state.update(&mut sprite_at_second_frame, &animation, smaller_duration);
            assert_eq!(sprite_at_second_frame.index, 1);
        }

        #[rstest]
        fn updates_index_if_less_than_expected_index(
            mut state: SpriteSheetAnimationState,
            mut sprite: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            smaller_duration: Duration,
        ) {
            state.update(&mut sprite, &animation, smaller_duration);
            assert_eq!(sprite.index, 1);
        }

        #[rstest]
        fn updates_index_if_greater_than_expected_index(
            mut state: SpriteSheetAnimationState,
            mut sprite_at_third_frame: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            smaller_duration: Duration,
        ) {
            state.update(&mut sprite_at_third_frame, &animation, smaller_duration);
            assert_eq!(sprite_at_third_frame.index, 1);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed(
            mut state: SpriteSheetAnimationState,
            mut sprite: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
        ) {
            state.update(&mut sprite, &animation, frame_duration);
            assert_eq!(sprite.index, 2);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed_after_multiple_updates(
            mut state: SpriteSheetAnimationState,
            mut sprite: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            smaller_duration: Duration,
        ) {
            state.update(&mut sprite, &animation, smaller_duration);
            state.update(&mut sprite, &animation, smaller_duration);
            assert_eq!(sprite.index, 2);
        }

        #[rstest]
        fn elapsed_duration_is_reset(
            mut state: SpriteSheetAnimationState,
            mut sprite: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
            smaller_duration: Duration,
        ) {
            state.update(&mut sprite, &animation, smaller_duration);
            state.update(&mut sprite, &animation, smaller_duration);
            assert_eq!(
                state.elapsed_in_frame,
                (smaller_duration + smaller_duration) - frame_duration
            );
        }

        #[rstest]
        fn returns_false(
            mut state: SpriteSheetAnimationState,
            mut sprite_at_second_frame: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
        ) {
            assert!(!state.update(&mut sprite_at_second_frame, &animation, frame_duration,));
        }

        #[rstest]
        fn skips_frame_if_too_much_time_elapsed(
            mut state: SpriteSheetAnimationState,
            mut sprite: TextureAtlasSprite,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
        ) {
            state.update(&mut sprite, &animation, frame_duration * 2);
            assert_eq!(sprite.index, 3);
        }
    }

    mod repeat_from {
        use crate::Frame;

        use super::*;

        mod on_last_frame {
            use super::*;

            #[fixture]
            fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
                SpriteSheetAnimation::from_frames(vec![
                    Frame::new(0, frame_duration),
                    Frame::new(1, frame_duration),
                    Frame::new(2, frame_duration),
                    Frame::new(3, frame_duration),
                    Frame::new(4, frame_duration),
                ])
                .repeat_from(2)
            }

            #[fixture]
            fn state() -> SpriteSheetAnimationState {
                SpriteSheetAnimationState {
                    current_frame: 4,
                    elapsed_in_frame: Duration::from_nanos(0),
                    going_backward: false,
                }
            }

            #[rstest]
            fn returns_to_loop_frame(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite_at_second_frame, &animation, frame_duration);
                assert_eq!(sprite_at_second_frame.index, 2);
            }

            #[rstest]
            fn returns_false(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                assert!(!state.update(&mut sprite_at_second_frame, &animation, frame_duration,));
            }
        }

        mod after_last_frame {
            use super::*;

            #[fixture]
            fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
                SpriteSheetAnimation::from_frames(vec![
                    Frame::new(0, frame_duration),
                    Frame::new(1, frame_duration),
                    Frame::new(2, frame_duration),
                    Frame::new(3, frame_duration),
                ])
                .repeat_from(2)
            }

            #[fixture]
            fn state() -> SpriteSheetAnimationState {
                SpriteSheetAnimationState {
                    current_frame: 4,
                    elapsed_in_frame: Duration::from_nanos(0),
                    going_backward: false,
                }
            }

            #[rstest]
            fn returns_to_first_frame(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite_at_second_frame, &animation, frame_duration);
                assert_eq!(sprite_at_second_frame.index, 2);
            }

            #[rstest]
            fn returns_false(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                assert!(!state.update(&mut sprite_at_second_frame, &animation, frame_duration,));
            }
        }
    }

    mod ping_pong {
        use super::*;

        mod on_last_frame {
            use super::*;

            #[fixture]
            fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
                SpriteSheetAnimation::from_range(0..=1, frame_duration).ping_pong()
            }

            #[fixture]
            fn state() -> SpriteSheetAnimationState {
                SpriteSheetAnimationState {
                    current_frame: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    going_backward: false,
                }
            }

            #[rstest]
            fn returns_to_previous_frame(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite_at_second_frame, &animation, frame_duration);
                assert_eq!(sprite_at_second_frame.index, 0);
            }

            #[rstest]
            fn changes_state_to_backward(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite_at_second_frame, &animation, frame_duration);
                assert!(state.going_backward);
            }
        }

        mod going_backward {
            use super::*;

            #[fixture]
            fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
                SpriteSheetAnimation::from_range(0..=2, frame_duration).ping_pong()
            }

            #[fixture]
            fn state() -> SpriteSheetAnimationState {
                SpriteSheetAnimationState {
                    current_frame: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    going_backward: true,
                }
            }

            #[rstest]
            fn continues_to_previous_frame(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite_at_second_frame, &animation, frame_duration);
                assert_eq!(sprite_at_second_frame.index, 0);
            }
        }
    }

    mod run_once {
        use super::*;

        #[fixture]
        fn animation(frame_duration: Duration) -> SpriteSheetAnimation {
            SpriteSheetAnimation::from_range(0..=1, frame_duration).once()
        }

        mod on_first_frame {
            use super::*;

            #[fixture]
            fn state() -> SpriteSheetAnimationState {
                SpriteSheetAnimationState {
                    current_frame: 0,
                    elapsed_in_frame: Duration::from_nanos(500),
                    going_backward: false,
                }
            }

            #[rstest]
            fn final_index_set_if_frames_skipped_past_end(
                mut state: SpriteSheetAnimationState,
                mut sprite: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite, &animation, frame_duration * 4);
                assert_eq!(sprite.index, 1);
            }
        }

        mod on_last_frame {
            use super::*;

            #[fixture]
            fn state() -> SpriteSheetAnimationState {
                SpriteSheetAnimationState {
                    current_frame: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    going_backward: false,
                }
            }

            #[rstest]
            fn does_nothing(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite_at_second_frame, &animation, frame_duration);
                assert_eq!(sprite_at_second_frame.index, 1);
            }

            #[rstest]
            fn returns_true(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                assert!(state.update(&mut sprite_at_second_frame, &animation, frame_duration,));
            }

            #[rstest]
            fn returns_to_initial_state(
                mut state: SpriteSheetAnimationState,
                mut sprite_at_second_frame: TextureAtlasSprite,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&mut sprite_at_second_frame, &animation, frame_duration);
                let expected_state = SpriteSheetAnimationState::default();
                assert_eq!(state.current_frame, expected_state.current_frame);
                assert_eq!(state.elapsed_in_frame, expected_state.elapsed_in_frame);
            }
        }
    }
}
