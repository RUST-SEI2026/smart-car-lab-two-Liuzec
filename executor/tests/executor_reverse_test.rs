use executor::{Executor, Pose};

#[test]
fn should_return_x_minus_1_given_status_is_reverse_command_is_m_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("BM");
    assert_eq!(Pose::new(-1, 0, 'E'), e.query());
}

#[test]
fn should_return_s_given_status_is_reverse_command_is_l_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("BL");
    assert_eq!(Pose::new(0, 0, 'S'), e.query());
}

#[test]
fn should_return_n_given_status_is_reverse_command_is_r_and_facing_is_e() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'E'));
    e.execute("BR");
    assert_eq!(Pose::new(0, 0, 'N'), e.query());
}

#[test]
fn should_return_y_plus_1_given_command_is_bbm_and_facing_is_n() {
    let mut e = Executor::with_pose(Pose::new(0, 0, 'N'));
    e.execute("BBM");
    assert_eq!(Pose::new(0, 1, 'N'), e.query());
}
