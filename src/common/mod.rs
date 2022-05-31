type StateFunctionType<StateType, ActionType> = Box<dyn Fn(StateType) -> (ActionType, StateType)>;

pub struct State<StateType, ActionType> {
    pub(crate) run_state: StateFunctionType<StateType, ActionType>,
}

/// ===========
/// Constructor
/// ===========
impl<StateType, ActionType> State<StateType, ActionType>
where
    StateType: 'static,
    ActionType: 'static + Clone,
{
    pub fn build_result(action: ActionType) -> Self {
        let f = Box::new(move |state| (action.clone(), state));
        Self { run_state: f }
    }
}

impl<StateType, ActionType> State<StateType, ActionType>
where
    StateType: 'static,
    ActionType: 'static,
{
    #[must_use]
    pub fn new(f: StateFunctionType<StateType, ActionType>) -> Self {
        Self { run_state: f }
    }

    #[must_use]
    pub fn build_union() -> State<StateType, ()> {
        let f = Box::new(move |state| ((), state));
        State::<StateType, ()>::new(f)
    }
}

/// running functions
impl<StateType, ActionType> State<StateType, ActionType>
where
    StateType: 'static,
    ActionType: 'static,
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

impl<StateType, ActionType> State<StateType, ActionType>
where
    StateType: Clone + 'static,
    ActionType: 'static,
{
    #[must_use]
    pub fn get(self) -> State<StateType, StateType> {
        let inner = self.run_state;
        State::<StateType, StateType>::new(Box::new(move |state: StateType| {
            let (_, inner_state) = inner(state);
            (inner_state.clone(), inner_state)
        }))
    }

    pub fn put(self, put_state: StateType) -> State<StateType, ()> {
        let inner = self.run_state;
        State::<StateType, ()>::new(Box::new(move |state: StateType| {
            let (_, _) = inner(state);
            ((), put_state.clone())
        }))
    }
}

impl<StateType, ActionType> State<StateType, ActionType>
where
    StateType: 'static,
    ActionType: 'static,
{
    #[must_use]
    pub fn modify(self, modifier: Box<dyn Fn(StateType) -> StateType>) -> State<StateType, ()> {
        let inner = self.run_state;
        State::<StateType, ()>::new(Box::new(move |state: StateType| {
            let (_, inner_state) = inner(state);
            let modified_state = modifier(inner_state);
            ((), modified_state)
        }))
    }
}
