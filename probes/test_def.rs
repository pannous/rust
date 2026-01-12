#!/usr/bin/env rustc
def meaning() int {return 42}
#def meaning() int {42}
#def meaning(){return 42}
def hello() {
	put!("Hello from def")
}

def main(){
	hello()
	put!("Meaning of life is %d\n", meaning())
}
