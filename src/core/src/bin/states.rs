/// A dummy implementation of the Circuit Breaker pattern to demonstrate
/// capabilities of this library.
/// https://martinfowler.com/bliki/CircuitBreaker.html
use mech_core::statemachines::*;
use mech_core::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use hashbrown::HashMap;

fn main() {

    let code = r#"
    traffic_light := { <1s>=>🟢=><6s>=>🟡=<10s>=>🔴=<10s>=>🟢}"#;




    let gn_id = hash_str("🟢");
    let yw_id = hash_str("🟡");
    let rd_id = hash_str("🔴");

    let gn_state = State::Id(gn_id);
    let yw_state = State::Id(yw_id);
    let rd_state = State::Id(rd_id);

    let mut machine: StateMachine = StateMachine::from_state(gn_state);
    machine.add_transition((gn_state, Event::TimerExpired),(yw_state,Output::SetTimer(6)));
    machine.add_transition((yw_state, Event::TimerExpired),(rd_state,Output::SetTimer(10)));
    machine.add_transition((rd_state, Event::TimerExpired),(gn_state,Output::SetTimer(10)));
    println!("{:#?}", machine);

    
    let mut event_queue: Vec<Event> = vec![];

    loop {




    }




/*
    // Unsuccessful request
    let machine = Arc::new(Mutex::new(machine));
    {
        let mut lock = machine.lock().unwrap();
        let res = lock.consume(Input::Unsuccessful).unwrap();
        assert_eq!(res, Output::SetTimer);
        assert_eq!(lock.state(), &State::Open);
    }


    
    // Set up a timer
    let machine_wait = machine.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::new(5, 0));
        let mut lock = machine_wait.lock().unwrap();
        let res = lock.consume(Input::TimerTriggered).unwrap();
        assert_eq!(res, Output::None);
        assert_eq!(lock.state(), &State::HalfOpen);
    });

    // Try to pass a request when the circuit breaker is still open
    let machine_try = machine.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::new(1, 0));
        let mut lock = machine_try.lock().unwrap();
        let res = lock.consume(Input::Successful);
        assert!(matches!(res, Err(TransitionError::Impossible)));
        assert_eq!(lock.state(), &State::Open);
    });

    // Test if the circit breaker was actually closed
    std::thread::sleep(Duration::new(7, 0));
    {
        let mut lock = machine.lock().unwrap();
        let res = lock.consume(Input::Successful).unwrap();
        assert_eq!(res, Output::None);
        assert_eq!(lock.state(), &State::Closed);
    }*/
    println!("Success!");
}