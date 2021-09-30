#### Goal

The simplest workflow:
- having a csv file from the google translate service
- having trello boards with english words
- parse the file and grouping the words according to the boards
- try to find the word on the board 
and if so then move it into the appropriate list, 
otherwise create a new one in the appropriate list  
- the info about bond between boards, list and csv file resides in the config file

#### Example
```shell
./trello-vocab-loader --cred ../../example/trello_token.json --cfg ../../example/cfg.json --data ../../example/data.csv
```

#### Structure of the files
Credential for trello:
```json
{
  "key": "key to trello api", 
  "token": "token to trello api"
}
```
Configuration:
- match_f: the factor affects how to coincide the word should be with the existing one.
- name: the value of the column in data.csv file
- board: the name of the board that respects to the column
- list_create: the name of the trello list in case if the new word needs to be created
- list_move: the name of the trello list in case if the word has a candidate to update and that word needs to move to the column
```json
{
  "dicts": [
    {
      "name": "German",
      "board": "GER",
      "list_create": "Later",
      "list_move": "Daily"
    }
  ],
  "match_f": 0.8
}
```
Data.csv:
- first column corresponds to the third one and the second one corresponds to the forth one respectively
```csv
English,Russian,sober,трезвый
English,Russian,tentative,пробный
English,Russian,snag,загвоздка
English,Russian,purge,удалять
```
