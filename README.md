#### Goal

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

### Take

```yaml
task:
  type: take
  params:
    from:
      type: pipe | board | column # pipe by default and all from can be omitted
      column: name
    size: number | all # all by default and size can be omitted
    place: top | bottom | random # top by default

task:
  type: order
  params:
    type: shuffle | sort | reverse
    from:
      type: pipe | column # pipe by default and all from can be omitted
      column: name

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
      place: top | bottom

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
