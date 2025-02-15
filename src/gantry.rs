
struct GantryMachine {
    x: i32,
    y: i32,
}

impl GantryMachine {
    fn new() -> GantryMachine {
        GantryMachine {
            x: 0,
            y: 0,
        }
    }

    fn move_x(&mut self, distance: i32) {
        self.x += distance;
    }

    fn move_y(&mut self, distance: i32) {
        self.y += distance;
    }
}

struct TheArm {
    gantry: GantryMachine,
    rotateZ: u16, // arm can rotate 
    positionZ: i32, // arm can move up and down
    extension: i32, // arm can extend
    handRotation: u16, // hand can rotate,
    handGripSet: [u16; 2], // hand can grip
}

impl TheArm {
    fn new() -> Self {
        TheArm {
            gantry: GantryMachine::new(),
            rotateZ: 0,
            positionZ: 0,
            extension: 0,
            handRotation: 0,
            handGripSet: [0, 0],
        }
    }
}

trait SafetySensor {
    fn distance(&self) -> u16;
}

impl SafetySensor for WristIR {
    
}