#### Description

A framework that enables to automatize some operations.
It grew from butler and other tools simply because the aforesaid tools did not provide the needed functions.
For me, as I use it predominantly for learning languages, the pipeline could be the following:

- take 10 random cards from the board ,that have a specific label
- move them to the specific column
- shuffle them down

or

- check the cards in column with a name '...'
- if a card follows a set of conditions then move it to the other column

#### Structure of the files

Credential for trello:

```json
{
  "key": "key to trello api", 
  "token": "token to trello api"
}
```

## Tasks

- take `number | all` from `column | board` where `top | bottom | random`
- shuffle | sort them
- filter them by
  - name | label
- move | copy to list

```yaml
task:
  type: take
  params:
    from:
      type: pipe | board | column # pipe by default and all from can be omitted
      source: name
    size: number | 0 # 0 by default and size can be omitted
    place: top | bottom | random  # top by default

task:
  type: order
  params:
    type: shuffle | sort | reverse
    from:
      type: pipe | column # pipe by default and all from can be omitted
      source: name

task: 
  type: filter
  params:
    by: name | label # name by default
    rhs: name
    case: false # by default  

task:
  type: action
  params:
    type: copy | move | print
    to: 
      column: name
      place: top | bottom | random 

task:
  type: group
  params:
    - task1      
    - task2      
    - task3

task:
  type: flow
  params:
    - task1      
    - task2      
    - task3       
```

### Arguments

The ability to pass some arguments in start like

```bash
args name=value name=value 
```

and process in tasks:

```yaml
filter_demand:
  type: filter
  params:
    by: label
    rhs: ~~demand~~

```
