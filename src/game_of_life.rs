
use setup::LifeRule;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::collections::HashSet;
//Cell with x and y
#[derive(Clone, Copy)]
pub struct CellCoordinates {
    pub x: u32,
    pub y: u32,
    pub alive: bool,
}
impl CellCoordinates{
    fn change_alive(&mut self,alive: bool) {
        self.alive = alive;
    }
}
impl PartialEq for CellCoordinates {
    fn eq(&self, other: &CellCoordinates) -> bool {
        return self.x == other.x && self.y == other.y && self.alive == other.alive
    }
}
// Cell with coordinate struct
pub struct Cell {
    pub coordinates: Coordinates,
    pub alive: bool,
}
impl Cell{
    fn change_alive(&mut self,alive: bool) {
        self.alive = alive;
    }
}
impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        return self.coordinates == other.coordinates && self.alive == other.alive
    }
}
#[derive(Eq, Hash, Debug, Clone)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Coordinates) -> bool {
        return self.x == other.x && self.y == other.y
    }
}

pub fn build_cell_coordinates(x: u32, y: u32, alive: bool) -> CellCoordinates {
    CellCoordinates {
        x: x,
        y: y,
        alive:alive
    }
}
fn count_neighbors(cells:&Vec<CellCoordinates>, x: u32, y: u32 ) -> u16{
        //if cell is alive and a neighbor count it as a neighbor
        return cells.into_iter().fold(0, |acc, cell| acc + (cell.alive && !(cell.x == x && cell.y == y) && (cell.x >= x-1 && cell.y >= y-1) && (cell.x <= x+1 && cell.y <= y+1)) as u16);

}
fn count_neighbors_from_hashmap(cells:&HashMap<Coordinates, bool>, x: u32, y: u32 ) -> u16{
        //if cell is alive and a neighbor count it as a neighbor
        let mut neighbors = 0;
        for i in x-1..x+2 {
            for j in y-1..y+2 {
                if !(i == x && j == y){

                    match cells.get(&Coordinates{x:i,y:j}){
                        Some(&alive) => if alive {neighbors += 1}
                        None    => {}
                    }
                }
            }
        }
        neighbors

}
pub fn create_next_generation_concurrent(previous_gen: &Vec<CellCoordinates>, rule: &LifeRule) -> Vec<CellCoordinates> {

    //vec.into_iter()
    //let mut next_gen: Vec<CellCoordinates> = Vec::new();
    let next_gen: Arc<Mutex<Vec<CellCoordinates>>>  = Arc::new(Mutex::new(Vec::new()));
    let previous_gen_con: Arc<Mutex<Vec<CellCoordinates>>>  = Arc::new(Mutex::new(previous_gen.clone()));
    let rule_con: Arc<Mutex<LifeRule>>  = Arc::new(Mutex::new(rule.clone()));
println!("Concurrent generation count {}", previous_gen.len());
    let _previous_gen = previous_gen.clone();
    let checked_set: Arc<Mutex<HashSet<Coordinates>>>  = Arc::new(Mutex::new( HashSet::new()));

    let mut handles = vec![];
    for k in 0..previous_gen.chunks(200).count()
    {
        let data_clone = previous_gen.clone();
        let next_gen = Arc::clone(&next_gen);
        let checked_set = Arc::clone(&checked_set);
        let previous_gen_con = Arc::clone(&previous_gen_con);
        let rule_con = Arc::clone(&rule_con);
        let handle = thread::spawn(move || {
            for chunk in data_clone.chunks(200).nth(k)
            {
                for coord  in chunk {
                    //println!("Coord{} {} {} {}",i, coord.x, coord.y, coord.alive);
                    if coord.alive && coord.x > 1 && coord.y > 1{
                    /*if !next_gen.iter().any(|cell| cell.x == coord.x && cell.y == coord.y ){
                        let neighbours = &count_neighbors(&previous_gen, coord.x, coord.y );
                        //let mut cell = coord.clone();
                        //cell.change_alive(rule.borns(*neighbours as usize));
                        next_gen.push(build_cell_coordinates(coord.x,coord.y,rule.borns(*neighbours as usize)));
                    }*/

                    for i in coord.x-1..coord.x+2 {
                        for j in coord.y-1..coord.y+2 {
                            let cell_coordinates = Coordinates{x:i,y:j};
                            let mut checked = checked_set.lock().unwrap();
                            //Skip if the cell exists already in the next gen
                            if !checked.contains(&cell_coordinates) && !(coord.x == i && coord.y == j) && !next_gen.lock().unwrap().iter().any(|cell| cell.x == i && cell.y == j ){
                                checked.insert(cell_coordinates);
                                //println!("Concurrent {} x:{},y:{}", k,i,j);
                            // Check if the cell is alive
                            //previous_gen_.iter().for_each(|cell| println!("Foreach {} {} {}", cell.x, cell.y, cell.alive));
                            let cell = data_clone.iter().find(|cell| cell.x == i && cell.y == j);

                            let neighbours = &count_neighbors(&previous_gen_con.lock().unwrap(), i, j );
                            match cell {
                                Some(v) => {
                                    if v.alive {
                                        let mut gen = next_gen.lock().unwrap();
                                        if rule_con.lock().unwrap().survives(*neighbours as usize){gen.push(build_cell_coordinates(i,j,true))};
                                        //next_gen.push(build_cell_coordinates(i,j,rule.survives(*neighbours as usize)));

                                    } else {
                                        let mut gen = next_gen.lock().unwrap();
                                        if rule_con.lock().unwrap().borns(*neighbours as usize){gen.push(build_cell_coordinates(i,j,true))};
                                        //next_gen.push(build_cell_coordinates(i,j,rule.borns(*neighbours as usize)));
                                    }
                                    //println!("Foreach {} {} {}", v.x, v.y, v.alive)
                                },
                                None => {
                                    //println!("None");
                                    let mut gen = next_gen.lock().unwrap();
                                    if rule_con.lock().unwrap().borns(*neighbours as usize){gen.push(build_cell_coordinates(i,j,true))};
                                },
                            }
                            /*let alive =  _previous_gen.iter().any(|cell| cell.x == i && cell.y == j && cell.alive);





                            //Where the being alive matters
                            if alive {
                                next_gen.push(build_cell_coordinates(i,j,rule.survives(*neighbours as usize)));

                            } else {

                                next_gen.push(build_cell_coordinates(i,j,rule.borns(*neighbours as usize)));
                            }*/

                            }
                        }
                    }
                }

                }

            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let gen = next_gen.lock().unwrap().clone();
    return gen;
}
pub fn create_next_generation_concurrent_hash(previous_gen: &HashMap<Coordinates, bool>, rule: &LifeRule) -> HashMap<Coordinates, bool> {

    let mut next_gen: HashMap<Coordinates, bool> = HashMap::new();
    println!("Genereation count {}", previous_gen.len());
    //println!("{:?}",previous_gen );
    let _previous_gen= &previous_gen;
    let mut checked_set: HashSet<Coordinates> =  HashSet::new();

    for (key, value)  in &*previous_gen {

        if *value && key.x > 1 && key.y > 1{
        /*if !next_gen.iter().any(|cell| cell.x == coord.x && cell.y == coord.y ){
            let neighbours = &count_neighbors(&previous_gen, coord.x, coord.y );
            //let mut cell = coord.clone();
            //cell.change_alive(rule.borns(*neighbours as usize));
            next_gen.push(build_cell_coordinates(coord.x,coord.y,rule.borns(*neighbours as usize)));
        }*/

        for i in key.x-1..key.x+2 {
            for j in key.y-1..key.y+2 {
                let cell = Coordinates {x:i,y:j};
                //Skip if the cell exists already in the next gen
                if !checked_set.contains(&cell) && !(key.x == i && key.y == j) && !next_gen.contains_key(&cell){
                checked_set.insert(cell.clone());
                // Check if the cell is alive
                //previous_gen_.iter().for_each(|cell| println!("Foreach {} {} {}", cell.x, cell.y, cell.alive));
                let previous_cell = _previous_gen.get(&cell);

                let neighbours = &count_neighbors_from_hashmap(&previous_gen, i, j );
                //println!("Neighbours {:?}",neighbours );
                match previous_cell {
                    Some(&v) => {
                        //println!("Alive {:?}",v );
                        if v {
                            if rule.survives(*neighbours as usize) {
                                next_gen.insert(cell,true);
                                //println!("Inserting {:?}",next_gen );
                            }
                            //next_gen.push(build_cell_coordinates(i,j,rule.survives(*neighbours as usize)));

                        } else {
                            if rule.borns(*neighbours as usize){
                                next_gen.insert(cell,true);
                                //println!("Inserting {:?}",next_gen );
                            }
                            //next_gen.push(build_cell_coordinates(i,j,rule.borns(*neighbours as usize)));
                        }
                        //println!("Foreach {} {} {}", v.x, v.y, v.alive)
                    },
                    None => {
                        //println!("None");
                        if rule.borns(*neighbours as usize){
                            next_gen.insert(cell,true);
                            //println!("Inserting {:?}",next_gen );
                        }
                    },
                }
                //println!("{:?}",next_gen );
                /*let alive =  _previous_gen.iter().any(|cell| cell.x == i && cell.y == j && cell.alive);





                //Where the being alive matters
                if alive {
                    next_gen.push(build_cell_coordinates(i,j,rule.survives(*neighbours as usize)));

                } else {

                    next_gen.push(build_cell_coordinates(i,j,rule.borns(*neighbours as usize)));
                }*/

                }
            }
        }
    }

    }


    return next_gen;
}
pub fn create_next_generation(previous_gen: &Vec<CellCoordinates>, rule: &LifeRule) -> Vec<CellCoordinates> {

    //vec.into_iter()
    let mut next_gen: Vec<CellCoordinates> = Vec::new();
    println!("Genereation count {}", previous_gen.len());
    let _previous_gen= &previous_gen;

    for coord  in previous_gen {

        if coord.alive && coord.x > 1 && coord.y > 1{
        /*if !next_gen.iter().any(|cell| cell.x == coord.x && cell.y == coord.y ){
            let neighbours = &count_neighbors(&previous_gen, coord.x, coord.y );
            //let mut cell = coord.clone();
            //cell.change_alive(rule.borns(*neighbours as usize));
            next_gen.push(build_cell_coordinates(coord.x,coord.y,rule.borns(*neighbours as usize)));
        }*/

        for i in coord.x-1..coord.x+2 {
            for j in coord.y-1..coord.y+2 {

                //Skip if the cell exists already in the next gen
                if !(coord.x == i && coord.y == j) && !next_gen.iter().any(|cell| cell.x == i && cell.y == j ){

                // Check if the cell is alive
                //previous_gen_.iter().for_each(|cell| println!("Foreach {} {} {}", cell.x, cell.y, cell.alive));
                let cell = _previous_gen.iter().find(|cell| cell.x == i && cell.y == j);

                let neighbours = &count_neighbors(&previous_gen, i, j );
                match cell {
                    Some(v) => {
                        if v.alive {
                            if rule.survives(*neighbours as usize){next_gen.push(build_cell_coordinates(i,j,true))};
                            //next_gen.push(build_cell_coordinates(i,j,rule.survives(*neighbours as usize)));

                        } else {
                            if rule.borns(*neighbours as usize){next_gen.push(build_cell_coordinates(i,j,true))};
                            //next_gen.push(build_cell_coordinates(i,j,rule.borns(*neighbours as usize)));
                        }
                        //println!("Foreach {} {} {}", v.x, v.y, v.alive)
                    },
                    None => {
                        //println!("None");
                        if rule.borns(*neighbours as usize){next_gen.push(build_cell_coordinates(i,j,true))};
                    },
                }


                }
            }
        }
    }

    }


    return next_gen;
}

#[cfg(test)]
mod tests {
    //use setup::LifeRule;
    #[test]
    fn game_of_life_generation() {
        let life_rule = ::setup::build_life_rule([false,false,false,true,false,false,false,false,false],
                                               [false,false,true,true,false,false,false,false,false]);
        let mut previous_gen: Vec<super::CellCoordinates> = Vec::new();
        previous_gen.push(super::build_cell_coordinates(10,11,true));
        previous_gen.push(super::build_cell_coordinates(10,12,true));
        previous_gen.push(super::build_cell_coordinates(10,13,true));
        let next_gen = super::create_next_generation(&previous_gen, &life_rule);
        let mut comparison: Vec<super::CellCoordinates> = Vec::new();
        comparison.push(super::build_cell_coordinates(9,12,true));
        comparison.push(super::build_cell_coordinates(10,12,true));
        comparison.push(super::build_cell_coordinates(11,12,true));
        for coord  in &comparison {

            assert_eq!(true,next_gen.iter().any(|cell| cell == coord));
        }
        //Concurrent test
        let mut previous_gen_con: Vec<super::CellCoordinates> = Vec::new();
        previous_gen_con.push(super::build_cell_coordinates(10,11,true));
        previous_gen_con.push(super::build_cell_coordinates(10,12,true));
        previous_gen_con.push(super::build_cell_coordinates(10,13,true));
        let next_gen_con = super::create_next_generation_concurrent(&previous_gen_con, &life_rule);
        let mut comparison_con: Vec<super::CellCoordinates> = Vec::new();
        comparison_con.push(super::build_cell_coordinates(9,12,true));
        comparison_con.push(super::build_cell_coordinates(10,12,true));
        comparison_con.push(super::build_cell_coordinates(11,12,true));
        for coord  in &comparison_con {

            assert_eq!(true,next_gen_con.iter().any(|cell| cell == coord));
        }

         assert_eq!(&3, &super::count_neighbors(&comparison, 10, 11));
         assert_eq!(&2, &super::count_neighbors(&comparison, 10, 12));
         let mut neighbor_test: Vec<super::CellCoordinates> = Vec::new();
         neighbor_test.push(super::build_cell_coordinates(9,12,true));
         neighbor_test.push(super::build_cell_coordinates(10,12,true));
         neighbor_test.push(super::build_cell_coordinates(11,12,true));
         neighbor_test.push(super::build_cell_coordinates(10,11,true));
         neighbor_test.push(super::build_cell_coordinates(10,13,true));

         //Testing hashmap
         let mut previous_gen_hash:super::HashMap<super::Coordinates,bool>  = super::HashMap::new();;
         previous_gen_hash.insert(super::Coordinates{x:10,y:11},true);
         previous_gen_hash.insert(super::Coordinates{x:10,y:12},true);
         previous_gen_hash.insert(super::Coordinates{x:10,y:13},true);
         let next_gen_con_hash = super::create_next_generation_concurrent_hash(&previous_gen_hash, &life_rule);
         println!("{:?}",next_gen_con_hash );
         let mut comparison_con_hash: super::HashMap<super::Coordinates,bool> = super::HashMap::new();
         comparison_con_hash.insert(super::Coordinates{x:9,y:12},true);
         comparison_con_hash.insert(super::Coordinates{x:10,y:12},true);
         comparison_con_hash.insert(super::Coordinates{x:11,y:12},true);
         for (key, value) in comparison_con_hash {
             assert_eq!(next_gen_con_hash.get(&key).is_some(), true);

         }
         assert_eq!(3, next_gen_con_hash.len());

         //assert_eq!(3, comparison.iter().zip(next_gen.iter()).filter(|&(a, b)| a == b).count());

    }
}
