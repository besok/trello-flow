use crate::err::FlowError;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub name: String,
    pub body: TaskBody,
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
    pub src: Source,
    pub size: usize,
    pub place: Place,
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
    pub column: String,
    pub place: Place,
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

impl OrderTask {
    pub fn source(&self) -> Result<Source, FlowError> {
        Ok(match self {
            OrderTask::Shuffle(s) | OrderTask::Sort(s) | OrderTask::Reverse(s) => s.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionTask {
    PrintToConsole,
    CopyToColumn(Target),
    MoveToColumn(Target),
    AddToColumn(CardInfo, Target),
}
#[derive(Debug, Clone, PartialEq)]
pub struct CardInfo {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterTask {
    Name(String, bool),
    Label(String, bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowTask {
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupTask {
    pub steps: Vec<String>,
}
