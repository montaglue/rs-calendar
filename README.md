# rs-calendare
This is test implementation of my time managment model. I want to extend it thet's why I need to fully understand and formalize it. Even my full version in Notion can't do even basic time and success rate prediction automaticaly add `Task`s. I would like to add fucntional like this to the app.

## Why rust
Becouse it is fun for me to write code in Rust.

## Combinatorical equation

We can look at the probem of making shedule as the combinatorical equation on some objects. Like we 
have tasks thtat should be done either in concreet time or until some deadline and in the finite amount of time we will have finite amount of taks (in infinite time it not alwayse true, because if you clean your teeth every day in infiti days you will brush your teeth inifty times). 

## Terminalogy
WorldState
WorldModel: Time -> WorldState
Sheduler: WorldModel -> WorldModel

## Todo
- [ ] Simplify Data
- [ ] Extend world model
- [ ] Add meta Flows
- [ ] Add Data saving to file
