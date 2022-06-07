type StateFunctionType<'a, StateType, ActionType> =
    Box<dyn Fn(StateType) -> (ActionType, StateType) + 'a>;

pub struct State<'a, StateType, ActionType> {
    pub(crate) run_state: StateFunctionType<'a, StateType, ActionType>,
}

/// ===========
/// Constructor
/// ===========

impl<'a, StateType, ActionType> State<'a, StateType, ActionType>
where
    StateType: 'a,
    ActionType: 'a,
{
    #[must_use]
    pub(crate) fn crate_intern_new<F>(f: F) -> Self
    where
        F: Fn(StateType) -> (ActionType, StateType) + 'a,
    {
        let boxed_f = Box::new(f);
        Self { run_state: boxed_f }
    }

    #[must_use]
    pub fn new() -> State<'a, StateType, ()> {
        let f = Box::new(move |state| ((), state));
        State::<StateType, ()>::crate_intern_new(f)
    }
}

/// =================
/// Running Functions
/// =================

impl<'a, StateType, ActionType> State<'a, StateType, ActionType>
where
    StateType: 'a,
    ActionType: 'a,
{
    pub fn run(&self, input: StateType) -> (ActionType, StateType) {
        (self.run_state)(input)
    }

    pub fn eval(&self, input: StateType) -> ActionType {
        self.run(input).0
    }

    pub fn exec(&self, input: StateType) -> StateType {
        self.run(input).1
    }
}

/// ===============
/// State Interface
/// ===============

impl<'a, StateType, ActionType> State<'a, StateType, ActionType>
where
    StateType: Clone + 'a,
    ActionType: 'a,
{
    #[must_use]
    pub fn get(self) -> State<'a, StateType, StateType> {
        let inner = self.run_state;
        State::<StateType, StateType>::crate_intern_new(Box::new(move |state: StateType| {
            let (_, inner_state) = inner(state);
            (inner_state.clone(), inner_state)
        }))
    }

    pub fn put(self, put_state: StateType) -> State<'a, StateType, ()> {
        let inner = self.run_state;
        State::<StateType, ()>::crate_intern_new(Box::new(move |state: StateType| {
            let (_, _) = inner(state);
            ((), put_state.clone())
        }))
    }
}

impl<'a, StateType, ActionType> State<'a, StateType, ActionType>
where
    StateType: 'a,
    ActionType: 'a,
{
    #[must_use]
    pub fn modify<F>(self, modifier: F) -> State<'a, StateType, ()>
    where
        F: Fn(StateType) -> StateType + 'a,
    {
        let boxed_modifier = Box::new(modifier);
        let inner = self.run_state;
        State::<StateType, ()>::crate_intern_new(Box::new(move |state: StateType| {
            let (_, inner_state) = inner(state);
            let modified_state = boxed_modifier(inner_state);
            ((), modified_state)
        }))
    }
}
