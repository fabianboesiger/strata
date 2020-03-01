use image::{DynamicImage};

pub mod load;

#[derive(Clone)]
struct Layer {
    pub position: (i32, i32),
    pub image: DynamicImage
}

impl Layer {
    fn new(image: DynamicImage) -> Layer {
        Layer {
            image,
            position: (0, 0)
        }
    }
}

trait Operation {
    fn apply(&self, view: View) -> View;
}

#[derive(Clone)]
struct View {
    pub layers: Vec<Layer>
}

struct OperationStep {
    operation: Box<dyn Operation>,
    preview: View
}

struct Operator {
    operations: Vec<OperationStep>
}

impl Operator {
    // Adds an operation to the operator.
    pub fn add<O: Operation + 'static>(&mut self, operation: O) {
        if self.operations.is_empty() {
            // Push initial operation.
            self.operations.push(OperationStep {
                operation: Box::new(operation),
                preview: View {
                    layers: Vec::new()
                }
            });
        } else {
            // Apply operation on preview and push it.
            self.operations.push(OperationStep {
                preview: operation.apply(self.operations.last().unwrap().preview.clone()),
                operation:  Box::new(operation)
            });
        }
    }

    pub fn reverse(&mut self) {
        self.operations.pop();
    }
}