use executor::{Executor, Pose};

#[test]
fn should_return_x_plus_2_given_status_is_fast_command_is_m_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("FM");
    assert_eq!(Pose::new(2, 0, 'E'), e.query());
}

#[test]
fn should_return_x_plus_1_and_facing_n_given_status_is_fast_command_is_l_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("FL");
    assert_eq!(Pose::new(1, 0, 'N'), e.query());
}

#[test]
fn should_return_x_plus_1_and_facing_s_given_status_is_fast_command_is_r_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("FR");
    assert_eq!(Pose::new(1, 0, 'S'), e.query());
}

#[test]
fn should_return_x_plus_1_given_command_is_ffm_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("FFM");
    assert_eq!(Pose::new(1, 0, 'E'), e.query());
}

#[test]
fn should_return_x_minus_2_given_status_is_bf_command_is_m_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("BFM");
    assert_eq!(Pose::new(-2, 0, 'E'), e.query());
}

#[test]
fn should_return_x_minus_1_and_facing_s_given_status_is_bf_command_is_l_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("BFL");
    assert_eq!(Pose::new(-1, 0, 'S'), e.query());
}

#[test]
fn should_return_x_minus_1_and_facing_n_given_status_is_bf_command_is_r_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("BFR");
    assert_eq!(Pose::new(-1, 0, 'N'), e.query());
}
