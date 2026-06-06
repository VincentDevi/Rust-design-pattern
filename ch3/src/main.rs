use std::cell::RefCell;

struct CalculationResult {
    expression: String,
    result: f64,
}

struct Calculator {
    history: RefCell<Vec<CalculationResult>>,
    current_expression: Option<String>,
}

trait HistoryViewer {
    fn view_history(&self) -> &RefCell<Vec<CalculationResult>>;
    fn get_last_result(&self) -> Option<f64>;
}

trait HistoryManager {
    fn add_to_history(&self, expression: String, result: f64);
    fn clear_history(&self);
}

impl Calculator {
    fn new() -> Self {
        Self {
            history: RefCell::new(Vec::new()),
            current_expression: None,
        }
    }
}

impl HistoryViewer for Calculator {
    fn view_history(&self) -> &RefCell<Vec<CalculationResult>> {
        &self.history
    }
    fn get_last_result(&self) -> Option<f64> {
        self.history.borrow().last().map(|r| r.result)
    }
}

impl HistoryManager for Calculator {
    fn clear_history(&self) {
        self.history.borrow_mut().clear();
    }
    fn add_to_history(&self, expression: String, result: f64) {
        self.history
            .borrow_mut()
            .push(CalculationResult { expression, result });
    }
}

fn main() {
    println!("Hello, world!");
}
