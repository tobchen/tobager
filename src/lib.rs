//! Game state management structs and traits.

/// A function that constructs a new state.
pub type StateConstructor<C> = fn(&mut C) -> Result<Box<dyn GameState<C>>, String>;

/// A game state manager.
/// 
/// A game state manager holds the game's context (for use across states) and the active game state.
/// It does not run the main loop itself, but instead the main loop is expected to invoke the
/// manager's `pass` method whenever considered appropriate (it's also up to the manager's user
/// to handle any timing duties like capping the frame rate).
pub struct GameManager<C> {
    /// The game's context.
    context: C,
    /// The active state.
    active_state: Box<dyn GameState<C>>,
    /// Whether the manager's still active.
    is_active: bool,
}

impl<C> GameManager<C> {
    /// Creates a new game state manager. Its active state will be created using the given state constructor.
    pub fn new(mut context: C, state_constructor: StateConstructor<C>) -> Result<GameManager<C>, String> {
        Ok(GameManager {
            active_state: match state_constructor(&mut context) {
                Ok(state) => state,
                Err(msg) => return Err(format!("Failed to create initial state: {}", &msg)),
            },
            context: context,
            is_active: true,
        })
    }

    /// Calls the active state's update method. Returns `false` if the active state told the manager to quit
    /// and deactivate.
    pub fn pass(&mut self) -> Result<bool, String> {
        if !self.is_active {
            return Ok(false);
        }

        match self.active_state.update_and_draw(&mut self.context) {
            GameStateUpdateResult::ChangeState(next_state_constructor) => {
                self.active_state.terminate(&mut self.context);
                self.active_state = match next_state_constructor(&mut self.context) {
                    Ok(state) => state,
                    Err(msg) => {
                        self.is_active = false;
                        return Err(format!("Failed to create next state: {}", &msg));
                    }
                };
            },
            GameStateUpdateResult::Quit => {
                self.active_state.terminate(&mut self.context);
                self.is_active = false;
                return Ok(false);
            },
            GameStateUpdateResult::Continue => ()
        }

        return Ok(true);
    }
}

/// A result of a manager's active state's pass.
pub enum GameStateUpdateResult<C> {
    /// Instructs the calling manager to continue using the active state in the next pass.
    Continue,
    /// Instructs the calling manager to terminate the active state and construct a new state
    /// to be set as active.
    ChangeState(StateConstructor<C>),
    /// Instructs the calling manager to terminate the active state and deactivate.
    Quit,
}

/// Methods for an active game state to be called by a game state manager.
pub trait GameState<C> {
    /// Updates the game world and renders the game. Called in a main loop pass.
    fn update_and_draw(&mut self, context: &mut C) -> GameStateUpdateResult<C>;
    /// Called when the state is going to be deactivated (and possibly disposed). Here,
    /// resources outside the Rust memory management (e.g. OpenGL buffers or textures)
    /// should be freed as this is the game state's last chance to do so.
    fn terminate(&mut self, context: &mut C);
}
