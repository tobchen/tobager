use tobager::*;
use std::io::stdin;

struct Context {
    switches: usize,
}

impl Context {
    fn new() -> Self {
        Context {
            switches: 0,
        }
    }
}

struct StateA {
    inputs: usize,
}

impl StateA {
    pub fn new(_: &mut Context) -> Result<Box<dyn GameState<Context>>, String> {
        Ok(Box::new(StateA {
            inputs: 0,
        }))
    }
}

impl GameState<Context> for StateA {
    fn update_and_draw(&mut self, context: &mut Context) -> GameStateUpdateResult<Context> {
        println!("I'm state A! Input-counter: {}; switch-counter: {}! 's' to switch, 'q' to quit!", self.inputs, context.switches);
        self.inputs += 1;
        let mut line = String::new();
        let _ = stdin().read_line(&mut line);
        match line.as_str() {
            "s\n" => GameStateUpdateResult::ChangeState(StateB::new),
            "q\n" => GameStateUpdateResult::Quit,
            _ => GameStateUpdateResult::Continue,
        }
    }

    fn terminate(&mut self, context: &mut Context) {
        context.switches += 1;
    }
}

struct StateB {
    inputs: usize,
}

impl StateB {
    pub fn new(_: &mut Context) -> Result<Box<dyn GameState<Context>>, String> {
        Ok(Box::new(StateB {
            inputs: 0,
        }))
    }
}

impl GameState<Context> for StateB {
    fn update_and_draw(&mut self, context: &mut Context) -> GameStateUpdateResult<Context> {
        println!("I'm state B! Input-counter: {}; switch-counter: {}! 's' to switch, 'q' to quit!", self.inputs, context.switches);
        self.inputs += 1;
        let mut line = String::new();
        let _ = stdin().read_line(&mut line);
        match line.as_str() {
            "s\n" => GameStateUpdateResult::ChangeState(StateA::new),
            "q\n" => GameStateUpdateResult::Quit,
            _ => GameStateUpdateResult::Continue,
        }
    }

    fn terminate(&mut self, context: &mut Context) {
        context.switches += 1;
    }
}

fn main() {
    let mut manager = GameManager::new(Context::new(), StateA::new).unwrap();
    loop {
        if !manager.pass().unwrap() {
            break;
        }
    }
}
