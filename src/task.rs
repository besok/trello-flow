pub mod context;
mod parse;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    name: String,
    body: TaskBody,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TaskBody {
    Take(TakeTask),
    Order(OrderTask),
    Filter(FilterTask),
    Action(ActionTask),
    Group(GroupTask),
    Flow(FlowTask),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TakeTask {
    src: Source,
    size: i64,
    place: Place,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Place {
    Top,
    Bottom,
    Random,
}

impl Default for Place {
    fn default() -> Self {
        Place::Top
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Target {
    column: String,
    place: Place,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Pipe,
    Board,
    Column(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderTask {
    Shuffle(Source),
    Sort(Source),
    Reverse(Source),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionTask {
    PrintToConsole,
    CopyToColumn(Target),
    MoveToColumn(Target),
}
#[derive(Debug, Clone, PartialEq)]
pub enum FilterTask {
    Name(String, bool),
    Label(String, bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowTask {
    steps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupTask {
    steps: Vec<String>,
}
