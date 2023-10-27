const SHIFT_PREFIX: &str = " -> ";
const SHIFT_PREFIX_LEN: usize = SHIFT_PREFIX.len();

const SPACE_PREFIX: &str = "    ";
const SPACE_PREFIX_LEN: usize = SPACE_PREFIX.len();

#[derive(Debug)]
pub struct ProguardMethod {
    pub deobfed: String,
    pub obfed: String,
    //pub sig: String,
}

impl ProguardMethod {
    fn deserialize(data: &String) -> Self {
        let focus = data.split(":").last().unwrap();

        let deobfed_index = focus.find(SHIFT_PREFIX).unwrap();
        let obfed_index = focus.len();
        
        let deobfed = focus.get(0..deobfed_index).unwrap();
        let obfed = focus.get(deobfed_index + SHIFT_PREFIX_LEN..obfed_index).unwrap();

        //let mut deobfed_sp = deobfed.split(" ");
        //println!("First: {}", deobfed_sp.next().unwrap());
        //println!("Last: {}", deobfed_sp.last().unwrap());

        Self {
            deobfed: String::from(deobfed),
            obfed: String::from(obfed),
            //sig: String::from(deobfed_sp.next().unwrap()).push(),
        }
    }

    fn copy(&self) -> Self {
        let mut deobfed = String::new();
        let mut obfed = String::new();
        deobfed.push_str(self.deobfed.as_str());
        obfed.push_str(self.obfed.as_str());

        Self {
            deobfed: deobfed,
            obfed: obfed
        }
    }
}

#[derive(Debug)]
pub struct ProguardField {
    pub deobfed: String,
    pub obfed: String,
    //pub sig: String,
}

impl ProguardField {
    fn deserialize(data: &String) -> Self {
        let data_len = data.len();
        let focus = data.get(SPACE_PREFIX_LEN..data_len).unwrap();

        
        let deobfed_index = focus.find(SHIFT_PREFIX).unwrap();
        let obfed_index = focus.len();
        
        let deobfed = focus.get(0..deobfed_index).unwrap();
        let obfed = focus.get(deobfed_index + SHIFT_PREFIX_LEN..obfed_index).unwrap();

        Self {
            deobfed: String::from(deobfed),
            obfed: String::from(obfed),
            //sig: String::new(),
        }
    }

    fn copy(&self) -> Self {
        let mut deobfed = String::new();
        let mut obfed = String::new();
        deobfed.push_str(self.deobfed.as_str());
        obfed.push_str(self.obfed.as_str());

        Self {
            deobfed: deobfed,
            obfed: obfed,
        }
    }
}

#[derive(Debug)]
pub struct ProguardClass {
    pub deobfed: String,
    pub obfed: String,
    pub fields: Vec<ProguardField>,
    pub methods: Vec<ProguardMethod>,
}

impl ProguardClass {
    pub fn deserialize(data: &Vec<String>) -> Self {
        let deobfed_index = data[0].find(SHIFT_PREFIX).unwrap();
        let obfed_index = data[0].len();
        
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        for (index, item) in data.iter().enumerate() {
            if index == 0 { continue; }

            if item.contains("(") {
                methods.push(ProguardMethod::deserialize(item));
            } else {
                fields.push(ProguardField::deserialize(item));
            }
        }

        Self {
            deobfed: String::from(data[0].get(0..deobfed_index).unwrap()),
            obfed: String::from(data[0].get(deobfed_index + SHIFT_PREFIX_LEN..obfed_index).unwrap()),
            fields: fields,
            methods: methods,
        }
    }

    pub fn copy(&self) -> Self {
        let mut deobfed = String::new();
        let mut obfed = String::new();
        deobfed.push_str(self.deobfed.as_str());
        obfed.push_str(self.obfed.as_str());
        
        let mut fields = Vec::new();
        self.fields.iter().for_each(|f| fields.push(f.copy()));
        let mut methods = Vec::new();
        self.methods.iter().for_each(|m| methods.push(m.copy()));

        Self {
            obfed: obfed,
            deobfed: deobfed,
            fields: fields,
            methods: methods,
        }
    }
}