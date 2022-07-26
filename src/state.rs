use std::time::Duration;

use crate::{animation::Mode, Frame, SpriteSheetAnimation};

/// Animation state component which is automatically inserted/removed
///
/// It can be used to reset the animation state.
///
/// # Example
#[cfg_attr(
    feature = "bevy-07",
    doc = "
```
# use bevy::prelude::*;
# use benimator::SpriteSheetAnimationState;
fn restart_anim_from_start(mut query: Query<&mut SpriteSheetAnimationState>) {
  for mut state in query.iter_mut() {
    state.reset();
  }
}
```
"
)]
#[derive(Default)]
pub struct SpriteSheetAnimationState {
    animation_frame_index: usize,
    sprite_frame_index: usize,
    elapsed_in_frame: Duration,
    // Control ping_pong backward frame navigation.
    going_backward: bool,
    is_ended: bool,
}

impl SpriteSheetAnimationState {
    /// Reset animation state
    ///
    /// The animation will restart from the first frame, like if the animation was freshly spawned.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    #[must_use]
    #[deprecated(since = "3.7.0", note = "please use `animation_frame_index` instead")]
    #[doc(hidden)]
    pub fn current_frame_index(&self) -> usize {
        self.animation_frame_index()
    }

    /// Returns the index of the current *animation* frame
    ///
    /// The index is relative to the animation sequence. **not** to the sprite-sheet.
    #[must_use]
    pub fn animation_frame_index(&self) -> usize {
        self.animation_frame_index
    }

    /// Returns the index of the current *sprite* frame
    ///
    /// The index is relative to the sprite atlas. **not** to the animation frame sequence.
    #[must_use]
    pub fn sprite_frame_index(&self) -> usize {
        self.sprite_frame_index
    }

    /// Returns true if the animation has ended
    #[must_use]
    pub fn is_ended(&self) -> bool {
        self.is_ended
    }

    #[must_use]
    fn frame<'a>(&self, animation: &'a SpriteSheetAnimation) -> &'a Frame {
        &animation.frames[self.animation_frame_index() % animation.frames.len()]
    }

    /// Update the animation state
    #[allow(dead_code)]
    pub fn update(&mut self, animation: &SpriteSheetAnimation, delta: Duration) {
        debug_assert!(animation.has_frames());
        let mut frame = self.frame(animation);
        self.sprite_frame_index = frame.index;
        self.elapsed_in_frame += delta;
        while self.elapsed_in_frame >= frame.duration {
            let on_last_frame = self.animation_frame_index >= animation.frames.len() - 1;
            match animation.mode {
                Mode::RepeatFrom(loop_from) => {
                    if on_last_frame {
                        self.animation_frame_index = loop_from;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                Mode::PingPong => {
                    if self.going_backward {
                        if self.animation_frame_index == 0 {
                            self.going_backward = false;
                            self.animation_frame_index += 1;
                        } else {
                            self.animation_frame_index -= 1;
                        }
                    } else if on_last_frame {
                        self.going_backward = true;
                        self.animation_frame_index -= 1;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                Mode::Once => {
                    if on_last_frame {
                        self.is_ended = true;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
            }

            self.elapsed_in_frame -= frame.duration;
            frame = self.frame(animation);
            self.sprite_frame_index = frame.index;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[fixture]
    fn frame_duration() -> Duration {
        Duration::from_secs(1)
    }

    #[fixture]
    fn smaller_duration(frame_duration: Duration) -> Duration {
        frame_duration - Duration::from_millis(1)
    }

    #[rstest]
    fn sprite_index(frame_duration: Duration) {
        let animation = SpriteSheetAnimation::from_range(3..=5, frame_duration);
        let mut state = SpriteSheetAnimationState::default();
        state.update(&animation, Duration::ZERO);
        assert_eq!(state.sprite_frame_index(), 3);
    }

    mod reset {
        use super::*;

        #[fixture]
        fn state() -> SpriteSheetAnimationState {
            SpriteSheetAnimationState {
                animation_frame_index: 1,
                elapsed_in_frame: Duration::from_secs(1),
                ..Default::default()
            }
        }

        #[rstest]
        fn resets_current_frame(mut state: SpriteSheetAnimationState) {
            state.reset();
            assert_eq!(state.animation_frame_index, 0);
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
            SpriteSheetAnimation::from_range(0..=2, frame_duration)
        }

        #[fixture]
        fn state() -> SpriteSheetAnimationState {
            SpriteSheetAnimationState::default()
        }

        #[rstest]
        fn nothing_happens_if_not_enough_time_has_elapsed_and_index_is_already_set(
            mut state: SpriteSheetAnimationState,
            animation: SpriteSheetAnimation,
            smaller_duration: Duration,
        ) {
            state.update(&animation, smaller_duration);
            assert_eq!(state.sprite_frame_index(), 0);
        }

        #[rstest]
        fn updates_index_if_less_than_expected_index(
            mut state: SpriteSheetAnimationState,
            frame_duration: Duration,
            smaller_duration: Duration,
        ) {
            let animation = SpriteSheetAnimation::from_range(1..=3, frame_duration);
            state.update(&animation, smaller_duration);
            assert_eq!(state.sprite_frame_index(), 1);
        }

        #[rstest]
        fn updates_index_if_greater_than_expected_index(
            mut state: SpriteSheetAnimationState,
            frame_duration: Duration,
            smaller_duration: Duration,
        ) {
            let animation = SpriteSheetAnimation::from_range(1..=3, frame_duration);
            state.update(&animation, smaller_duration);
            assert_eq!(state.sprite_frame_index(), 1);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed(
            mut state: SpriteSheetAnimationState,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
        ) {
            state.update(&animation, frame_duration);
            assert_eq!(state.sprite_frame_index(), 1);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed_after_multiple_updates(
            mut state: SpriteSheetAnimationState,
            animation: SpriteSheetAnimation,
            smaller_duration: Duration,
        ) {
            state.update(&animation, smaller_duration);
            state.update(&animation, smaller_duration);
            assert_eq!(state.sprite_frame_index(), 1);
        }

        #[rstest]
        fn elapsed_duration_is_reset(
            mut state: SpriteSheetAnimationState,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
            smaller_duration: Duration,
        ) {
            state.update(&animation, smaller_duration);
            state.update(&animation, smaller_duration);
            assert_eq!(
                state.elapsed_in_frame,
                (smaller_duration + smaller_duration) - frame_duration
            );
        }

        #[rstest]
        fn is_not_ended(
            mut state: SpriteSheetAnimationState,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
        ) {
            state.update(&animation, frame_duration);
            assert!(!state.is_ended());
        }

        #[rstest]
        fn skips_frame_if_too_much_time_elapsed(
            mut state: SpriteSheetAnimationState,
            animation: SpriteSheetAnimation,
            frame_duration: Duration,
        ) {
            state.update(&animation, frame_duration * 2);
            assert_eq!(state.sprite_frame_index(), 2);
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
                    animation_frame_index: 4,
                    elapsed_in_frame: Duration::from_nanos(0),
                    ..Default::default()
                }
            }

            #[rstest]
            fn returns_to_loop_frame(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.sprite_frame_index(), 2);
            }

            #[rstest]
            fn is_not_ended(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert!(!state.is_ended());
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
                    animation_frame_index: 4,
                    elapsed_in_frame: Duration::from_nanos(0),
                    ..Default::default()
                }
            }

            #[rstest]
            fn returns_to_first_frame(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.sprite_frame_index(), 2);
            }

            #[rstest]
            fn is_not_ended(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert!(!state.is_ended());
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
                    animation_frame_index: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    ..Default::default()
                }
            }

            #[rstest]
            fn returns_to_previous_frame(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.sprite_frame_index(), 0);
            }

            #[rstest]
            fn changes_state_to_backward(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
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
                    animation_frame_index: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    going_backward: true,
                    ..Default::default()
                }
            }

            #[rstest]
            fn continues_to_previous_frame(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.sprite_frame_index(), 0);
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
                    animation_frame_index: 0,
                    elapsed_in_frame: Duration::from_nanos(500),
                    ..Default::default()
                }
            }

            #[rstest]
            fn final_index_set_if_frames_skipped_past_end(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration * 4);
                assert_eq!(state.sprite_frame_index(), 1);
            }
        }

        mod on_last_frame {
            use super::*;

            #[fixture]
            fn state() -> SpriteSheetAnimationState {
                SpriteSheetAnimationState {
                    animation_frame_index: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    ..Default::default()
                }
            }

            #[rstest]
            fn does_nothing(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.sprite_frame_index(), 1);
            }

            #[rstest]
            fn is_ended(
                mut state: SpriteSheetAnimationState,
                animation: SpriteSheetAnimation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert!(state.is_ended());
            }
        }
    }
}
