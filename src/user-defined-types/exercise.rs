// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(dead_code)]

// ANCHOR: solution
// ANCHOR: event
#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    // ANCHOR_END: event
    /// A button was pressed.
    ButtonPressed(Button),

    /// Driver pushes the break pedal.
    BreakPedalPushed(Percentage),

    /// The car's doors have opened.
    CarDoorOpened(Door),

    /// The car's doors have closed.
    CarDoorClosed(Door),

    /// Car completely stopped
    CarStopped
}

// ANCHOR: Percentage
/// The push of the break pedal is represented by a floating-point percentage.
type Percentage = f32;
// ANCHOR_END: Percentage


// ANCHOR: Button
#[derive(Debug)]
/// A user-accessible button.
enum Button {
    VolumenUp,
    VolumenDown
}
// ANCHOR_END: Button

// ANCHOR: Door
#[derive(Debug)]
/// Door of the car.
enum Door {
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight
}
// ANCHOR_END: Door

// ANCHOR: car_door_opened
/// The car doors have opened.
fn car_door_opened(door: Door) -> Event {
    // ANCHOR_END: car_door_opened
    Event::CarDoorOpened(door)
}

// ANCHOR: car_door_closed
/// The car doors have closed.
fn car_door_closed(door: Door) -> Event {
    // ANCHOR_END: car_door_closed
    Event::CarDoorClosed(door)
}

// ANCHOR: car_stopped
/// The car doors have stopped.
fn car_stopped() -> Event {
    // ANCHOR_END: car_stopped
    Event::CarStopped
}

// ANCHOR: passenger_button_volumeup
/// A passenger has pressed the volume up button.
fn passenger_button_volumeup() -> Event {
    // ANCHOR_END: passenger_button_volumeup
    Event::ButtonPressed(Button::VolumenUp)
}

// ANCHOR: break_pedal_pressed
/// The driver has pressed the break pedal.
fn break_pedal_pressed(amount: Percentage) -> Event {
    // ANCHOR_END: break_pedal_pressed
    Event::BreakPedalPushed(amount)
}

// ANCHOR: main
fn main() {
    println!(
        "Passenger pressed a button: {:?}",
        passenger_button_volumeup()
    );

    println!(
        "Driver breaks: {:?}",
        break_pedal_pressed(0.79)
    );

    println!("The car has stopped: {:?}", car_stopped());
    
    println!(
        "Door opened: {:?}",
        car_door_opened(Door::FrontLeft)
    );
    println!("The car door closed: {:?}", car_door_closed(Door::FrontLeft));
}
// ANCHOR_END: main
