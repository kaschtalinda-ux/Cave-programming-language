use std :: io;
use std :: fs;

//use rand :: thread_rng;
//use rand :: Rng;




fn get_file(file: String) -> String {
    let code: String = fs :: read_to_string(format!("{}.cave", file .trim() .to_string())).expect("File not found...");

    return code;

}















fn is_right(str_: &str, chars: &str) -> bool {



    for chr in str_.chars() {
        if chars.contains(chr) {

        } else {
            return false;
        }
        
    }


    









    return true;




}








fn get_value(value_: Vec<String>, var: Vec<Vec<String>>) -> String {

    let mut value: Vec<String> = value_;


    





    if value.len() == 1 {

        if value[0] == "getchar" {
            let mut input_char = String :: new();

            io :: stdin() .read_line(&mut input_char) .expect("");
            
            let char_ = input_char .chars() .next() .unwrap_or(' ');

            value[0] = (char_ as i64) .to_string();
        }/* else if value[0] == "rand" {

            let mut random_int = rand :: thread_rng();
            value[0]  = random_int .gen_range(2..101) .to_string();
        }*/




    for var_ in var.clone() {
        if var_[0] == value[0] {

            value = vec![var_[var_[1].parse :: <usize> () .expect("")].clone()];
            break;
        }


    }

    }




if value.len() == 1 {
    if !is_right(&value[0], "1234567890") {
        panic!("invalid syntax")
    }
    return value[0].clone();
}











    if value[1] == "+" {



        return ((get_value(vec![value[0].to_string()], var.clone()).parse :: <i64>()).expect("invalid value") + (get_value(vec![value[2].to_string()], var.clone()).parse :: <i64>()).expect("invalid value")).to_string();
    }



    if value[1] == "-" {



        return ((get_value(vec![value[0].to_string()], var.clone()).parse :: <i64>()).expect("invalid value") - (get_value(vec![value[2].to_string()], var.clone()).parse :: <i64>()).expect("invalid value")).to_string();
    }



    if value[1] == "*" {



        return ((get_value(vec![value[0].to_string()], var.clone()).parse :: <i64>()).expect("invalid value") * (get_value(vec![value[2].to_string()], var.clone()).parse :: <i64>()).expect("invalid value")).to_string();
    }



    if value[1] == "/" {



        return ((get_value(vec![value[0].to_string()], var.clone()).parse :: <i64>()).expect("invalid value") / (get_value(vec![value[2].to_string()], var.clone()).parse :: <i64>()).expect("invalid value")).to_string();
    }



    if value[1] == "%" {



        return ((get_value(vec![value[0].to_string()], var.clone()).parse :: <i64>()).expect("invalid value") % (get_value(vec![value[2].to_string()], var.clone()).parse :: <i64>()).expect("invalid value")).to_string();
    }



    
    if value[1] == "<" {



        let mut bool_: bool = ((get_value(vec![value[0].to_string()], var.clone())) .parse :: <i64>() .expect("invalid value")) < ((get_value(vec![value[2].to_string()], var.clone()).parse :: <i64>()).expect("invalid value"));

        if bool_ {
            return 1.to_string();
        } else {
            return 0.to_string();
        }



    }





panic!("invalid value {:?}", value);





return "".to_string();


}














fn main() {


println!("welcome to Cave...\n\n");

let mut file = String :: new();
 


print!("File:\n");




io :: stdin() .read_line(&mut file) .expect("");





let mut strcode: String = get_file(file);


println!("\n\n\n\n\n\n\n\n");


let mut veccode: Vec<String> = strcode.split("\n").map(|s| s.to_string()). collect();




let mut code: Vec<Vec<String>> = vec![];


let mut s_line_: Vec<String>;


let mut codeline: Vec<String>;

let mut splitvalue: Vec<String>;


let mut iscom: bool;

let mut line_: String;



for line_o in veccode {
    codeline = vec![];
    iscom = false;
    line_ = line_o.trim().to_string();



    if line_ == "" {
        continue;
    }




    for chr in line_.chars() {
        if chr == '/' {
            iscom = true;
        }


        break;
    }


    if iscom {
        continue;
    }



    s_line_ = line_.split(" >> ").map(|s| s.to_string()). collect();



    if s_line_.len() != 2 {
        panic!("invalid line {}", line_);
    }








    codeline.push(s_line_[0].clone());



    splitvalue = s_line_[1].split(" ").map(|s| s.to_string()). collect();

    if splitvalue.len() == 1 {

    } else if splitvalue.len() == 3 {



    } else {
        panic!("invalid value");
    }




    for object in splitvalue {

        codeline.push(object.to_string());






    }    




    code.push(codeline);



}


















let mut lt: u16;




let mut loops: Vec<u16> = vec![];



let mut var: Vec<Vec<String>> = vec![];


let mut line: Vec<String> = vec![];


let mut value: Vec<String>;


let mut t: u16 = 0;

let mut obj_ptr: u8;


let mut new_var: bool;


let mut var_ptr: u16;

let mut set_ptr: u16;


let mut loop_safe: u8;







println!("###\n{:?}\n###", code);




while (t as usize) < code.len() {

    line = code[t as usize].clone();
    value = vec![];

    obj_ptr = 0;


    for obj in line.clone() {

        if obj_ptr == 0 {
            obj_ptr += 1;

            continue;
        }


        value.push(obj);
        obj_ptr += 1;
    }




    match line[0].as_str() {


        "putchar" => {
            
            let mut out_char: u8 = get_value(value.clone(), var.clone()) .parse :: <u8>() .expect("invalid value");
            
            print!("{}", out_char as char);
        
        
        
        
        },


        "putint" => print!("{}", get_value(value.clone(), var.clone())),



        "while" => {








            if get_value(value.clone(), var.clone()) != "0" {


                loops.push(t);



            } else {

            lt = t + 1;
            loop_safe = 1;


            while true {
                if (lt as usize) == code.len() {
                    panic!("while loop has no 'goto >> <bool>'");
                }

                if code[lt as usize][0] == "while" {
                    loop_safe += 1;
                }


                if code[lt as usize][0] == "goto" {
                    loop_safe -= 1;
                    

                    if loop_safe == 0 {

                        break;
                    }
                }

                lt += 1;
            }

                t = lt;
            }








        },



        "exit" => break,



        "goto" => {
            if loops.len() == 0 {
                panic!("invalud 'goto'");
            }

            if get_value(value.clone(), var.clone()) != "0" {
                t = loops[loops.len() - 1];

            } else {
                loops.pop();
            }





        },





        "addtoptr" => {
            set_ptr = 0;

            for i in var.clone() {


                if i[0] == value[0] {


                    if i.len() - 1 != var[set_ptr as usize][1] .parse :: <usize> () .expect("") {
                        var[set_ptr as usize][1] = (var[set_ptr as usize][1] .parse :: <usize> () .expect("") + 1) .to_string();


                    } else {
                        panic!("can not adding 1 from the ptr from variable '{}'", i[0]);
                    }
                }


                set_ptr += 1;

            }
        },



        "deltoptr" => {
            set_ptr = 0;

            for i in var.clone() {


                if i[0] == value[0] {


                    if 2 != var[set_ptr as usize][1] .parse :: <usize> () .expect("") {
                        var[set_ptr as usize][1] = (var[set_ptr as usize][1] .parse :: <usize> () .expect("") - 1) .to_string();


                    } else {
                        panic!("can not delite 1 from the ptr from variable '{}'", i[0]);
                    }
                }


                set_ptr += 1;

            }
        },





        "push" => {

            set_ptr = 0;
            for i in var.clone() {


                if i[0] == value[0] {

                    var[set_ptr as usize].push("0".to_string());

                    break;
                }

                set_ptr += 1;
            }

        },












        
        "pop" => {

            set_ptr = 0;
            for i in var.clone() {


                if i[0] == value[0] {

                    if i.len() != 3 {
                        var[set_ptr as usize].pop();
                    } else {
                        panic!("can not pop a element from variable '{}'", i[0]);
                    }

                    break;
                }

                set_ptr += 1;
            }

        },










        _ => {
            new_var = true;

            for var1 in var.clone() {
                if  var1[0] == line[0] { 

                    new_var = false;
                }

            }

            if new_var {
                var.push(vec![line[0].clone(), "2".to_string(), "".to_string()]);
            }




            var_ptr = 0;

            for var1 in var.clone() {

                if var1[0] == line[0] {
                    var[var_ptr as usize][var1[1].parse :: <usize> () .expect("")] = get_value(value.clone(), var.clone());
                }


                var_ptr += 1;
            }





        },


    }





    t += 1;
}



























println!("\n");

}
