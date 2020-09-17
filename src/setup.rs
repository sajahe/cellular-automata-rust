pub struct Setup {
    mode: String,
}
#[derive(Clone)]
pub struct LifeRule{
    birth_rule: [bool;9],
    survival_rule: [bool;9],
}
impl LifeRule {
    pub fn borns(&self,neighbours: usize) -> bool {
        self.birth_rule[neighbours]
    }
    pub fn survives(&self,neighbours: usize) -> bool {
        self.survival_rule[neighbours]
    }
}

pub fn build_life_rule(birth_rule: [bool;9],survival_rule:[bool;9]) -> LifeRule{
    LifeRule {
        birth_rule:birth_rule,
        survival_rule:survival_rule,
    }
}
/*
fn build_life_rule(rule: String){
    let v: Vec<&str> = "B35678/S5678".split('/').collect();
    let first = v.next();

    LifeRule {
        birth_rule:birth_rule,
        survival_rule:survival_rule,
    }
}*/
