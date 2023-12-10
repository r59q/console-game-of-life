use console_engine::KeyCode;
use console_engine::MouseButton;
use std::time::Duration;

use crate::input_manager::axis::Axis::Horizontal;
use crate::input_manager::axis::Axis::Vertical;
use crate::input_manager::buttons::Button;
use crate::input_manager::input_action::InputAction;
use crate::input_manager::input_types::InputType::Key;

use crate::resources::inputs::axis_inputs::AxisInputs;
use crate::resources::inputs::button_inputs::ButtonInputs;
use crate::resources::inputs::input_bindings::InputBindings;
use crate::resources::inputs::mouse_inputs::MouseInputs;

use crate::resources::positioned_entities::PositionedEntities;
use crate::resources::render_targets::RenderTargets;
use crate::resources::timer::Timer;
use crate::resources::view_offset::ViewOffset;
use crate::systems::pause_toggle::pause_toggle;
use crate::systems::reset_axis_input::reset_axial_inputs;
use crate::systems::reset_mouse_input::reset_mouse_inputs;
use crate::systems::timing::timing_system;

use super::*;

#[test]
fn can_add_timer_resource() {
    let mut test_env = initialize();

    let time_resource = Timer::new();

    test_env.game.get_world_mut().insert_resource(time_resource);

    let time = test_env.game.get_world_mut().get_resource::<Timer>();

    match time {
        None => {
            assert!(false)
        }
        Some(_) => {
            assert!(true)
        }
    }
}

#[test]
fn timer_duration_changes_over_time() {
    let mut test_env = initialize();

    let time_resource = Timer::new();

    test_env.game.get_world_mut().insert_resource(time_resource);

    let time = test_env.game.get_world_mut().get_resource::<Timer>();

    let first_time_duration = std::time::SystemTime::now()
        .duration_since(time.unwrap().start_time)
        .unwrap();
    std::thread::sleep(Duration::from_micros(100));
    let second_time_duration = std::time::SystemTime::now()
        .duration_since(time.unwrap().start_time)
        .unwrap();
    assert_ne!(
        first_time_duration.as_micros(),
        second_time_duration.as_micros()
    )
}

#[test]
fn timer_with_timing_system_contains_last_frame_delta() {
    let test_env = initialize();
    let mut game = test_env.game;
    let time_resource = Timer::new();
    game.get_world_mut().insert_resource(time_resource);
    let timing_system = timing_system;
    let mut schedule = Schedule::default();

    schedule.add_stage("timer", SystemStage::parallel().with_system(timing_system));

    let world_mut = game.get_world_mut();

    let mut time = world_mut.get_resource::<Timer>();

    let first_frame_delta = time.unwrap().delta_time.as_nanos();

    schedule.run(world_mut);
    schedule.run(world_mut);

    time = world_mut.get_resource::<Timer>();

    let second_frame_delta = time.unwrap().delta_time.as_nanos();

    schedule.run(world_mut);

    time = world_mut.get_resource::<Timer>();

    let third_frame_delta = time.unwrap().delta_time.as_nanos();

    assert_eq!(first_frame_delta, 0);
    assert_ne!(second_frame_delta, 0);
    assert_ne!(second_frame_delta, third_frame_delta);
}

#[test]
fn can_add_rendering_target() {
    let mut test_env = initialize();

    let render_targets_resource = RenderTargets::new();

    test_env
        .game
        .get_world_mut()
        .insert_resource(render_targets_resource);

    let render_targets = test_env
        .game
        .get_world_ref()
        .get_resource::<RenderTargets>();

    match render_targets {
        None => {
            assert!(false)
        }
        Some(_) => {
            assert!(true)
        }
    }
}

#[test]
fn can_add_input_resource() {
    let mut test_env = initialize();

    let input_resource = AxisInputs::new();

    test_env
        .game
        .get_world_mut()
        .insert_resource(input_resource);

    let inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>();

    assert!(matches!(inputs, Some(_)));
}

#[test]
fn can_read_from_input_resource() {
    let mut test_env = initialize();

    let input_resource = AxisInputs::new();

    test_env
        .game
        .get_world_mut()
        .insert_resource(input_resource);

    let inputs = test_env
        .game
        .get_world_ref()
        .get_resource::<AxisInputs>()
        .unwrap();

    let input_value = inputs.get(&Horizontal);

    assert_eq!(0., input_value)
}

#[test]
fn can_set_input_resource() {
    let mut test_env = initialize();
    let input_resource = AxisInputs::new();
    test_env
        .game
        .get_world_mut()
        .insert_resource(input_resource);

    let mut inputs_mut = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<AxisInputs>()
        .unwrap();
    inputs_mut.set(Horizontal, 1.);

    let inputs = test_env
        .game
        .get_world_ref()
        .get_resource::<AxisInputs>()
        .unwrap();
    assert_eq!(inputs.get(&Horizontal), 1.);
}

#[test]
fn system_can_reset_axis_inputs() {
    let mut test_env = initialize();
    let input_resource = AxisInputs::new();
    test_env
        .game
        .get_world_mut()
        .insert_resource(input_resource);

    test_env.game.add_stage_to_schedule(
        "test",
        SystemStage::parallel().with_system(reset_axial_inputs),
    );

    let mut inputs_mut = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<AxisInputs>()
        .unwrap();
    inputs_mut.set(Horizontal, 1.);

    let mut inputs = test_env
        .game
        .get_world_ref()
        .get_resource::<AxisInputs>()
        .unwrap();

    assert_eq!(inputs.get(&Horizontal), 1.);

    test_env.game.run_schedule();

    inputs = test_env
        .game
        .get_world_ref()
        .get_resource::<AxisInputs>()
        .unwrap();

    assert_eq!(0., inputs.get(&Horizontal));
}

#[test]
fn can_add_axis_inputs() {
    let mut test_env = initialize();
    let input_resource = AxisInputs::new();

    test_env
        .game
        .get_world_mut()
        .insert_resource(input_resource);

    test_env.game.add_stage_to_schedule(
        "test",
        SystemStage::parallel().with_system(reset_axial_inputs),
    );

    let inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>();
    assert!(matches!(inputs, Some(_)));
}

#[test]
fn can_add_mouse_inputs() {
    let mut test_env = initialize();
    let input_resource = MouseInputs::new();

    test_env
        .game
        .get_world_mut()
        .insert_resource(input_resource);

    let inputs = test_env.game.get_world_ref().get_resource::<MouseInputs>();
    assert!(matches!(inputs, Some(_)));
}

#[test]
fn system_can_reset_mouse_inputs() {
    let mut test_env = initialize();
    let input_resource = MouseInputs::new();

    test_env
        .game
        .get_world_mut()
        .insert_resource(input_resource);

    test_env.game.add_stage_to_schedule(
        "test",
        SystemStage::parallel().with_system(reset_mouse_inputs),
    );

    let mut inputs = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<MouseInputs>()
        .unwrap();

    inputs.set_state(MouseButton::Left, InputAction::Down, 1, 2);

    test_env.game.run_schedule();

    let hopefully_reset_state = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<MouseInputs>()
        .unwrap();

    assert_eq!(hopefully_reset_state.get_position(), (1, 2));
    let state = hopefully_reset_state.get_state(MouseButton::Left);
    assert_eq!(state.get_action(), InputAction::None);
}

#[test]
fn can_add_key_bindings() {
    let mut test_env = initialize();
    let keybinding_resource = InputBindings::new();

    test_env
        .game
        .get_world_mut()
        .insert_resource(keybinding_resource);

    let inputs = test_env
        .game
        .get_world_ref()
        .get_resource::<InputBindings>();
    assert!(matches!(inputs, Some(_)));
}

#[test]
fn can_add_key_to_key_bindings() {
    let mut test_env = initialize();
    let mut keybinding_resource = InputBindings::new();

    keybinding_resource.bind_to_axis(Horizontal, Key(KeyCode::Char('d')), Key(KeyCode::Char('a')));

    test_env
        .game
        .get_world_mut()
        .insert_resource(keybinding_resource);

    let inputs = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<InputBindings>();
    assert!(matches!(inputs, Some(_)));
    let keybindings = inputs.unwrap();

    let horizontals = keybindings.get_axial_bindings(Horizontal);

    assert_eq!(horizontals.unwrap().len(), 1);

    let verticals = keybindings.get_axial_bindings(Vertical);
    assert!(matches!(verticals, None));
}

#[test]
fn can_add_button_inputs_resource() {
    let mut test_env = initialize();
    let button_bindings = ButtonInputs::new();
    test_env
        .game
        .get_world_mut()
        .insert_resource(button_bindings);

    let inputs = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<ButtonInputs>();
    assert!(matches!(inputs, Some(_)));
}

#[test]
fn can_add_view_offset_resource() {
    let mut test_env = initialize();
    let view_offset = ViewOffset::new();
    test_env.game.get_world_mut().insert_resource(view_offset);

    let offset_resource = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<ViewOffset>();
    assert!(matches!(offset_resource, Some(_)));
}

#[test]
fn can_set_view_offset_resource() {
    let mut test_env = initialize();
    let view_offset = ViewOffset::new();
    test_env.game.get_world_mut().insert_resource(view_offset);

    let inputs = test_env
        .game
        .get_world_mut()
        .get_resource_mut::<ViewOffset>();
    if let Some(mut resource) = inputs {
        resource.set_offset(22, -23)
    } else {
        panic!("Could not find resource")
    }

    let result = test_env.game.get_world_mut().get_resource::<ViewOffset>();
    if let Some(resource) = result {
        let offset = resource.get_offset();
        assert_eq!((22, -23), offset)
    } else {
        panic!("Could not find resource")
    }
}

#[test]
fn can_pause_by_button() {
    let mut test_env = initialize();
    test_env
        .game
        .get_world_mut()
        .insert_resource(ButtonInputs::new());
    test_env
        .game
        .get_world_mut()
        .insert_resource(PauseState::new());

    test_env.game.add_stage_to_schedule(
        "progress",
        SystemStage::parallel().with_system(pause_toggle),
    );

    test_env
        .game
        .get_world_mut()
        .get_resource_mut::<ButtonInputs>()
        .unwrap()
        .set_btn(Button::Pause, InputAction::Down);

    let pause_state1 = test_env
        .game
        .get_world_ref()
        .get_resource::<PauseState>()
        .unwrap();

    assert!(!pause_state1.is_paused());

    test_env.game.run_schedule();

    let pause_state2 = test_env
        .game
        .get_world_ref()
        .get_resource::<PauseState>()
        .unwrap();

    assert!(pause_state2.is_paused());
}

#[test]
fn can_see_positioned_entities() {
    let mut test_env = initialize();

    test_env
        .game
        .get_world_mut()
        .insert_resource(PositionedEntities::new());

    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. });

    test_env.game.add_stage_to_schedule(
        "stage_label",
        SystemStage::parallel().with_system(positioned_entities_updater),
    );

    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>();

    let positioned_entity_some = positioned_entities.unwrap().get(&(0, 0));
    let positioned_entity_none = positioned_entities.unwrap().get(&(1, 1));
    let neighbours_of_none = positioned_entities
        .unwrap()
        .get_neighbours_of_position((1, 1));

    assert!(positioned_entity_some.is_some());
    assert!(positioned_entity_none.is_none());
    assert_eq!(1, neighbours_of_none.len());
}

#[test]
fn can_toggle_help_menu_state() {
    let mut test_env = initialize();

    test_env
        .game
        .get_world_mut()
        .insert_resource(HelpMenuState::new());

    test_env
        .game
        .get_world_mut()
        .insert_resource(UILayer::new());

    test_env.game.add_stage_to_schedule(
        "stage_label",
        SystemStage::parallel().with_system(help_menu_toggeling),
    );

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Help, InputAction::Down);
    test_env.game.get_world_mut().insert_resource(button_inputs);

    test_env.game.run_schedule();

    let help_menu_state = test_env
        .game
        .get_world_ref()
        .get_resource::<HelpMenuState>()
        .unwrap();

    assert!(!help_menu_state.is_visible())
}
