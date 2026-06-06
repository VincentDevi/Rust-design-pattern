struct CalculationResult {
    expression: String,
    result: f64,
}

struct Calculator {
    history: Vec<CalculationResult>,
    current_expression: Option<String>,
}

struct HistoryView<'a> {
    entries: &'a [CalculationResult],
}

impl Calculator {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            current_expression: None,
        }
    }

    fn view_hitsory(&self) -> &[CalculationResult] {
        &self.history
    }

    fn get_last_result(&self) -> Option<f64> {
        self.history.last().map(|r| r.result)
    }

    fn add_to_history(&mut self, expression: String, result: f64) {
        self.history.push(CalculationResult { expression, result });
    }

    fn clear_history(&mut self) {
        self.history.clear();
    }

    fn calculate_expression(&self, expression: &str) -> f64 {
        todo!()
    }

    fn create_history_view(&self) -> HistoryView<'_> {
        HistoryView {
            entries: &self.history,
        }
    }

    fn evaluate(&mut self, expression: String) -> Result<f64, String> {
        let result = self.calculate_expression(&expression);
        self.add_to_history(expression, result);
        Ok(result)
    }
}

fn main() {
    println!("Hello, world!");
}
