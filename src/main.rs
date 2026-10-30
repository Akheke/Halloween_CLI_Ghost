use crate::utils::ask;

mod utils;

fn main() {


    let default_gohst = r"
         [ghost]
         _______
       /         |
      |   O  O   |  << Trick or Treat!!
      /     v    ]]
     /_________/
        
    ";

    let shine_gohst = r"
         [ghost]
         _______
       /         |
      |   O  O   |  << Really !?
      /  *  v *  ]]
     /_________/
        
    ";

    let happy_ghost = r"
         [ghost]
         _______
       /         |
      |   O  O   |  << bye~
      /  *  v *  ]] 
     /_________/
        
    "; 

    let angry_ghost = r"
         [ghost]
         _______
       / #       |
      |   `  '   |  <<  You're so cheap!!
      /         ]]
     /_________/
        
    ";

    let scary_ghost = r"
     ________     __________    __________    __________    ____     ____    __
    |   __   |   |  ______  |  |  ______  |  |  ______  |  |    |   |    |  |  |
    |  |__|  |   | |      | |  | |      | |  | |      | |  | | | |_| | | |  |  |
    |   ___   |  | |      | |  | |      | |  | |      | |  | |  |   |  | |  |__|
    |  |___|  |  | |______| |  | |______| |  | |______| |  | |   |_|   | |   __
    |_________|  |__________|  |__________|  |__________|  |_|         |_|  |__|


                                         _______
                                       /        |
                                     <|  `  '   |>
                                      |_________|
    ";

    let question_ghost = r"
         [ghost]
         _______
       /         |
      |   O  O   |  <<  Surprised?
      /  *  v *  ]] 
     /_________/
        
    ";

    let great_ghost = r"
         [ghost]
         _______
       /         |
      |   ^  ^   |  <<  yay!
      /  *  v *  ]] 
     /_________/
        
    ";

    let goodbye_ghost = r"
         [ghost]
         _______
       /         |
      |   O  O   |  <<  have a nice day!
      /  *  v *  ]] 
     /_________/
        
    ";

    let shocked_ghost = r"
         [ghost]
         _______   !!
       /         |
      |   O  O   |
      /          ]] 
     /_________/
        
    ";

    let crying_ghost = r"
         [ghost]
         _______
       /         |
      |   -  -   |  <<  I'll come back even scarier..!
      /   :  :   ]] 
     /_________/
        
    ";

    let mut prompt = String::from(default_gohst);
    let mut select = [String::from("treat"), String::from("trick")];
    let err_msg = String::from("ew...\nMom! I messed up  ;(");
    let result = utils::ask(&prompt, &select, &err_msg);
    //trick or treat
    if result == 0 {
      // treat
        prompt = String::from(shine_gohst);
        select = [String::from("sure"), String::from("no")];
        let result2 = utils::ask(&prompt, &select, &err_msg);
        if result2 == 0 {
          // sure
          prompt = String::from(happy_ghost);
          utils::clear(&err_msg);
          println!("{}", prompt);
          utils::sleep(2000);
          println!("\x1b[0m");
          utils::clear(&err_msg);
        } else if result2 == 1 {
          // no
          prompt = String::from(scary_ghost);
          utils::trick(&prompt, 100, 50, &err_msg);
          prompt = String::from(angry_ghost);
          utils::clear(&err_msg);
          println!("{}",prompt);
          utils::sleep(5000);
          utils::clear(&err_msg);
        }
    } else if result == 1 {
      //trick
      prompt = String::from(scary_ghost);
      utils::trick(&prompt, 100, 30, &err_msg);
      println!("\x1b[0m");
      utils::sleep(500);
      prompt = String::from(question_ghost);
      select = [String::from("yes"), String::from("no")];
      let result3 = ask(&prompt, &select, &err_msg);
      if result3 == 0 {
        //yes
        prompt = String::from(great_ghost);
        utils::clear(&err_msg);
        println!("{}", prompt);
        utils::sleep(2000);
        prompt = String::from(goodbye_ghost);
        utils::clear(&err_msg);
        println!("{}", prompt);
        utils::sleep(2000);
        println!("\x1b[0m");
        utils::clear(&err_msg);
      } else if result3 == 1 {
        //no
        prompt = String::from(shocked_ghost);
        utils::clear(&err_msg);
        println!("{}", prompt);
        utils::sleep(2000);
        utils::clear(&err_msg);
        prompt = String::from(crying_ghost);
        println!("{}", prompt);
        utils::sleep(2500);
        utils::clear(&err_msg);
      }
    }
    utils::clear(&err_msg);
}
