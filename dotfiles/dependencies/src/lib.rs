use std::sync::RwLock;


#[test]
fn validate_design_one() {

    #[derive(Debug)]
    struct Program<'a> {
        name: String,
        parents: RwLock<Vec<&'a Program<'a>>>,
        children: RwLock<Vec<&'a Program<'a>>>,
    }
    
    impl<'a> Program<'a> {
        fn new(name: &str) -> Program<'a> {
            Program {
                name: name.to_string(),
                parents: RwLock::new(Vec::new()),
                children: RwLock::new(Vec::new()),
            }
        }

        fn add_child(&self, child: &'a Program<'a>) {
            self.children.write().unwrap().push(child);
        }
    }

    let docker = Program::new("docker");
    let dep_a = Program::new("dep_a");
    let dep_b = Program::new("dep_b");

    dep_a.add_child(&dep_b);
    docker.add_child(&dep_a);
    docker.add_child(&dep_b);

    println!("{:?}", docker);
}

#[test]
fn validate_design_two() {

    #[derive(Debug)]
    struct Program<'a> {
        name: String,
        parents: RwLock<Vec<&'a Program<'a>>>,
        children: RwLock<Vec<&'a Program<'a>>>,
    }

    impl<'a> Program<'a> {
        fn new(name: &str) -> Program<'a> {
            Program {
                name: name.to_string(),
                parents: RwLock::new(Vec::new()),
                children: RwLock::new(Vec::new()),
            }
        }

        fn add_child(&'a self, child: &'a Program<'a>) {
            self.children.write().unwrap().push(child);
            child.parents.write().unwrap().push(self);
        }
    }

    let docker = Program::new("docker");
    let dep_a = Program::new("dep_a");
    let dep_b = Program::new("dep_b");

    docker.add_child(&dep_a);
    docker.add_child(&dep_b);
    dep_a.add_child(&dep_b);

    assert_eq!(docker.children.read().unwrap().len(), 2);
    assert_eq!(dep_a.children.read().unwrap().len(), 1);
    assert_eq!(dep_b.children.read().unwrap().len(), 0);

    assert_eq!(docker.parents.read().unwrap().len(), 0);
    assert_eq!(dep_a.parents.read().unwrap().len(), 1);
    assert_eq!(dep_b.parents.read().unwrap().len(), 2);

    assert_eq!(docker.children.read().unwrap()[0].name, "dep_a");
    assert_eq!(docker.children.read().unwrap()[1].name, "dep_b");
    assert_eq!(dep_a.parents.read().unwrap()[0].name, "docker");
    assert_eq!(dep_b.parents.read().unwrap()[0].name, "docker");
    assert_eq!(dep_b.parents.read().unwrap()[1].name, "dep_a");
}

#[test]
fn validate_design_three() {

    #[derive(Debug)]
    struct Program<'a> {
        name: String,
        parents: RwLock<Vec<&'a Program<'a>>>,
        children: RwLock<Vec<&'a Program<'a>>>,
        enabled: RwLock<bool>,
    }

    impl<'a> Program<'a> {
        fn new(name: &str) -> Program<'a> {
            Program {
                name: name.to_string(),
                parents: RwLock::new(Vec::new()),
                children: RwLock::new(Vec::new()),
                enabled: RwLock::new(false),
            }
        }

        fn add_child(&'a self, child: &'a Program<'a>) {
            self.children.write().unwrap().push(child);
            child.parents.write().unwrap().push(self);
        }

        fn disable(&self) {
            *self.enabled.write().unwrap() = false;
        }

        fn enable(&self) {
            *self.enabled.write().unwrap() = true;
        }

        fn is_available(&self) -> bool {
            // check if we are enabled, and all children recursively are enabled
            let enabled = self.enabled.read().unwrap();
            if !*enabled {
                return false;
            }

            let children = self.children.read().unwrap();
            for child in children.iter() {
                if !child.is_available() {
                    return false;
                }
            }

            true
        }
    }

    let docker = Program::new("docker");
    let dep_a = Program::new("dep_a");
    let dep_b = Program::new("dep_b");
    let dep_c = Program::new("dep_c");

    docker.add_child(&dep_a);
    docker.add_child(&dep_b);
    dep_a.add_child(&dep_b);
    dep_b.add_child(&dep_c);


    // disable dep_c, should make all unavailable
    dep_c.disable();
    assert!(!docker.is_available());
    assert!(!dep_a.is_available());
    assert!(!dep_b.is_available());
    assert!(!dep_c.is_available());
    dep_c.enable();

    // disable dep_b, should make all EXCEPT dep_c unavailable
    dep_b.disable();
    assert!(!docker.is_available());
    assert!(!dep_a.is_available());
    assert!(!dep_b.is_available());
    assert!(dep_c.is_available());
    dep_b.enable();

    // disable dep_a, should make all EXCEPT dep_c and b unavailable
    dep_a.disable();
    assert!(!docker.is_available());
    assert!(!dep_a.is_available());
    assert!(dep_b.is_available());
    assert!(dep_c.is_available());
    dep_a.enable();

    // disable docker, should make only docker unavailable
    docker.disable();
    assert!(!docker.is_available());
    assert!(dep_a.is_available());
    assert!(dep_b.is_available());
    assert!(dep_c.is_available());
    docker.enable();
}