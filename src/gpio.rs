use std::{sync::mpsc, time::Duration};
use rppal::gpio::{Result, Gpio, Trigger};

use crate::event::Event;

#[derive(Clone, Debug)]
pub enum Button {
    Up = 17,
    Down = 22,
    Select = 23,
    Back = 27,
}

impl Button {
    fn from_u8(n: u8) -> Self {
        match n {
            17 => Button::Up,
            22 => Button::Down,
            23 => Button::Select,
            _ => Button::Back,
        }
    }

    fn list() -> Vec<Self> {
        vec![Button::Up, Button::Down, Button::Select, Button::Back]
    }
}

#[derive(Debug)]
pub struct GpioHandler {
    sender: mpsc::Sender<Event>,
}

/// Thread for handling Pi GPIO input events.
impl GpioHandler {

    pub fn new(sender: mpsc::Sender<Event>) -> Self {
        Self { sender }
    }

    /// Runs the GPIO event thread.
    pub fn run(self) -> Result<()> {
        let gpio = Gpio::new()?;
        let mut pins: Vec<_> = Vec::new();
        for button in Button::list().iter().by_ref() {
            let mut pin = gpio.get(button.clone() as u8)?.into_input_pullup();
            let _ = pin.set_interrupt(Trigger::FallingEdge, Some(Duration::from_millis(50)));
            pins.push(pin);
        }

        let pins_ref: Vec<_> = pins.iter().collect();
        let timeout = Duration::from_millis(100);
        loop {

            if let Ok(Some((p, _evt))) = gpio.poll_interrupts(pins_ref.iter().as_slice(), false, Some(timeout)) {
                self.send(Event::Gpio(Button::from_u8(p.pin())));
            }
        }
    }

    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    
}