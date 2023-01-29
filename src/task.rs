pub mod context;

enum Task {
    Take(TakeTask),
    Order(OrderTask),
    Filter(FilterTask),
    Action(ActionTask),
    Group(GroupTask),
    Flow(FlowTask),
}

struct TakeTask {
    src: Source,
    size: usize,
    place: Place,
}

enum Place {
    Top,
    Bottom,
    Random,
}

enum Source {
    Pipe,
    Board,
    Column(String),
}

enum OrderTask {
    Shuffle(Source),
    Sort(Source),
    Reverse(Source),
}

enum ActionTask {
    PrintToConsole,
    CopyToColumn(String, Place),
    MoveToColumn(String, Place),
}

enum FilterTask {
    Name(String, bool),
    Label(String, bool),
}

struct FlowTask {
    steps: Vec<String>,
}
struct GroupTask {
    steps: Vec<String>,
}
