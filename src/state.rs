use std::time::Duration;

use crate::{animation::Mode, Animation, Frame};

/// Animation state
#[derive(Default, Clone)]
pub struct State {
    animation_frame_index: usize,
    sprite_frame_index: usize,
    elapsed_in_frame: Duration,
    /// Control ping_pong backward frame navigation.
    going_backward: bool,
    is_ended: bool,
}

impl State {
    /// Create a new state
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset animation state
    ///
    /// The animation will restart from the first frame, like if the animation was freshly spawned.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns the current frame index
    #[must_use]
    pub fn frame_index(&self) -> usize {
        self.sprite_frame_index
    }

    /// Returns true if the animation has ended
    #[must_use]
    pub fn is_ended(&self) -> bool {
        self.is_ended
    }

    #[must_use]
    fn frame<'a>(&self, animation: &'a Animation) -> &'a Frame {
        &animation.frames[self.animation_frame_index % animation.frames.len()]
    }

    /// Update the animation state
    #[allow(dead_code)]
    pub fn update(&mut self, animation: &Animation, delta: Duration) {
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
    use crate::FrameRate;

    #[fixture]
    fn frame_duration() -> Duration {
        Duration::from_secs(1)
    }

    #[fixture]
    fn frame_rate(frame_duration: Duration) -> FrameRate {
        FrameRate::from_frame_duration(frame_duration)
    }

    #[fixture]
    fn smaller_duration(frame_duration: Duration) -> Duration {
        frame_duration - Duration::from_millis(1)
    }

    #[rstest]
    fn sprite_index(frame_rate: FrameRate) {
        let animation = Animation::from_indices(3..=5, frame_rate);
        let mut state = State::default();
        state.update(&animation, Duration::ZERO);
        assert_eq!(state.frame_index(), 3);
    }

    mod reset {
        use super::*;

        #[fixture]
        fn state() -> State {
            State {
                animation_frame_index: 1,
                elapsed_in_frame: Duration::from_secs(1),
                ..Default::default()
            }
        }

        #[rstest]
        fn resets_current_frame(mut state: State) {
            state.reset();
            assert_eq!(state.animation_frame_index, 0);
        }

        #[rstest]
        fn resets_elapsed_time(mut state: State) {
            state.reset();
            assert_eq!(state.elapsed_in_frame, Duration::ZERO);
        }
    }

    mod on_first_frame {
        use super::*;

        #[fixture]
        fn animation(frame_rate: FrameRate) -> Animation {
            Animation::from_indices(0..=2, frame_rate)
        }

        #[fixture]
        fn state() -> State {
            State::default()
        }

        #[rstest]
        fn nothing_happens_if_not_enough_time_has_elapsed_and_index_is_already_set(
            mut state: State,
            animation: Animation,
            smaller_duration: Duration,
        ) {
            state.update(&animation, smaller_duration);
            assert_eq!(state.frame_index(), 0);
        }

        #[rstest]
        fn updates_index_if_less_than_expected_index(
            mut state: State,
            frame_rate: FrameRate,
            smaller_duration: Duration,
        ) {
            let animation = Animation::from_indices(1..=3, frame_rate);
            state.update(&animation, smaller_duration);
            assert_eq!(state.frame_index(), 1);
        }

        #[rstest]
        fn updates_index_if_greater_than_expected_index(
            mut state: State,
            frame_rate: FrameRate,
            smaller_duration: Duration,
        ) {
            let animation = Animation::from_indices(1..=3, frame_rate);
            state.update(&animation, smaller_duration);
            assert_eq!(state.frame_index(), 1);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed(
            mut state: State,
            animation: Animation,
            frame_duration: Duration,
        ) {
            state.update(&animation, frame_duration);
            assert_eq!(state.frame_index(), 1);
        }

        #[rstest]
        fn updates_index_if_enough_time_has_elapsed_after_multiple_updates(
            mut state: State,
            animation: Animation,
            smaller_duration: Duration,
        ) {
            state.update(&animation, smaller_duration);
            state.update(&animation, smaller_duration);
            assert_eq!(state.frame_index(), 1);
        }

        #[rstest]
        fn elapsed_duration_is_reset(
            mut state: State,
            animation: Animation,
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
        fn is_not_ended(mut state: State, animation: Animation, frame_duration: Duration) {
            state.update(&animation, frame_duration);
            assert!(!state.is_ended());
        }

        #[rstest]
        fn skips_frame_if_too_much_time_elapsed(
            mut state: State,
            animation: Animation,
            frame_duration: Duration,
        ) {
            state.update(&animation, frame_duration * 2);
            assert_eq!(state.frame_index(), 2);
        }
    }

    mod repeat_from {
        use crate::Frame;

        use super::*;

        mod on_last_frame {
            use super::*;

            #[fixture]
            fn animation(frame_duration: Duration) -> Animation {
                Animation::from_frames(vec![
                    Frame::new(0, frame_duration),
                    Frame::new(1, frame_duration),
                    Frame::new(2, frame_duration),
                    Frame::new(3, frame_duration),
                    Frame::new(4, frame_duration),
                ])
                .repeat_from(2)
            }

            #[fixture]
            fn state() -> State {
                State {
                    animation_frame_index: 4,
                    elapsed_in_frame: Duration::from_nanos(0),
                    ..Default::default()
                }
            }

            #[rstest]
            fn returns_to_loop_frame(
                mut state: State,
                animation: Animation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.frame_index(), 2);
            }

            #[rstest]
            fn is_not_ended(mut state: State, animation: Animation, frame_duration: Duration) {
                state.update(&animation, frame_duration);
                assert!(!state.is_ended());
            }
        }

        mod after_last_frame {
            use super::*;

            #[fixture]
            fn animation(frame_duration: Duration) -> Animation {
                Animation::from_frames(vec![
                    Frame::new(0, frame_duration),
                    Frame::new(1, frame_duration),
                    Frame::new(2, frame_duration),
                    Frame::new(3, frame_duration),
                ])
                .repeat_from(2)
            }

            #[fixture]
            fn state() -> State {
                State {
                    animation_frame_index: 4,
                    elapsed_in_frame: Duration::from_nanos(0),
                    ..Default::default()
                }
            }

            #[rstest]
            fn returns_to_first_frame(
                mut state: State,
                animation: Animation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.frame_index(), 2);
            }

            #[rstest]
            fn is_not_ended(mut state: State, animation: Animation, frame_duration: Duration) {
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
            fn animation(frame_rate: FrameRate) -> Animation {
                Animation::from_indices(0..=1, frame_rate).ping_pong()
            }

            #[fixture]
            fn state() -> State {
                State {
                    animation_frame_index: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    ..Default::default()
                }
            }

            #[rstest]
            fn returns_to_previous_frame(
                mut state: State,
                animation: Animation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.frame_index(), 0);
            }

            #[rstest]
            fn changes_state_to_backward(
                mut state: State,
                animation: Animation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert!(state.going_backward);
            }
        }

        mod going_backward {
            use super::*;

            #[fixture]
            fn animation(frame_rate: FrameRate) -> Animation {
                Animation::from_indices(0..=2, frame_rate).ping_pong()
            }

            #[fixture]
            fn state() -> State {
                State {
                    animation_frame_index: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    going_backward: true,
                    ..Default::default()
                }
            }

            #[rstest]
            fn continues_to_previous_frame(
                mut state: State,
                animation: Animation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration);
                assert_eq!(state.frame_index(), 0);
            }
        }
    }

    mod run_once {
        use super::*;

        #[fixture]
        fn animation(frame_rate: FrameRate) -> Animation {
            Animation::from_indices(0..=1, frame_rate).once()
        }

        mod on_first_frame {
            use super::*;

            #[fixture]
            fn state() -> State {
                State {
                    animation_frame_index: 0,
                    elapsed_in_frame: Duration::from_nanos(500),
                    ..Default::default()
                }
            }

            #[rstest]
            fn final_index_set_if_frames_skipped_past_end(
                mut state: State,
                animation: Animation,
                frame_duration: Duration,
            ) {
                state.update(&animation, frame_duration * 4);
                assert_eq!(state.frame_index(), 1);
            }
        }

        mod on_last_frame {
            use super::*;

            #[fixture]
            fn state() -> State {
                State {
                    animation_frame_index: 1,
                    elapsed_in_frame: Duration::from_nanos(500),
                    ..Default::default()
                }
            }

            #[rstest]
            fn does_nothing(mut state: State, animation: Animation, frame_duration: Duration) {
                state.update(&animation, frame_duration);
                assert_eq!(state.frame_index(), 1);
            }

            #[rstest]
            fn is_ended(mut state: State, animation: Animation, frame_duration: Duration) {
                state.update(&animation, frame_duration);
                assert!(state.is_ended());
            }
        }
    }
}
