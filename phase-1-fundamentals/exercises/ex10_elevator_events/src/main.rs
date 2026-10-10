#![allow(dead_code)]
#[derive(Debug)]
// Enum for the event in the elevator
enum Event{
    // A button was pressed
    ButtonPressed(Button),
    // The car has arrived at the third floor
    CarArrived(Floor),
    // The car's door opened 
    CarDoorOpened,
    // The car's door have closed 
    CarDoorClosed,
}
type Floor = i32;

// A Direction to travel
#[derive(Debug)]
enum Direction{
    Up,
    Down,
}
#[derive(Debug)]
enum Button{
    LobbyCall(Direction, Floor),
    // A floor button within the car
    CarFloor(Floor),
}
// FUNCTIONS 
fn car_arrived(floor:i32)->Event{
    Event::CarArrived(floor)
}
fn car_door_opened()->Event{
    Event::CarDoorOpened
}
fn car_door_closed()->Event{
    Event::CarDoorClosed
}
fn lobby_call_button_pressed(floor:i32,dir:Direction)->Event{
    Event::ButtonPressed(Button::LobbyCall(dir,floor))
}
fn car_floor_button_pressed(floor:i32)->Event{
    Event::ButtonPressed(Button::CarFloor(floor))
}
fn main(){
    println!("A ground floor passenger has pressed the up button: {:?}", lobby_call_button_pressed(0,Direction::Up));
    println!("The car door opened: {:?}",car_door_opened());
    println!("A passenger has pressed the third floor button: {:?}",car_floor_button_pressed(3));
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car arrived arrived at the third floor: {:?}",car_arrived(3));

}