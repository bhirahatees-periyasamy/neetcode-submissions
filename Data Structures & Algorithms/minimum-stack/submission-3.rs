struct MinStack {
    stack: Vec<i32>,
    min: i32,
    top: i32
}

impl MinStack {
    // initializes the stack object.
    pub fn new() -> Self {
        MinStack { stack: Vec::new(), min: i32::MAX, top: 0 }
    }

    // pushes the element val onto the stack.
    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        self.min = self.min.min(val);
        self.top = val;
    }
    
    // removes the element on the top of the stack.
    pub fn pop(&mut self) {
        self.stack.pop();
        self.min = *self.stack.iter().min().unwrap_or(&i32::MAX);
        self.top = *self.stack.last().unwrap_or(&0);
    }

    // gets the top element of the stack.
    pub fn top(&self) -> i32 {
        self.top
    }

    // retrieves the minimum element in the stack.
    pub fn get_min(&self) -> i32 {
        self.min
    }
}
