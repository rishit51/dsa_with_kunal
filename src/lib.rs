use std::{f64::consts::PI, intrinsics::sqrtf64, io};


pub fn isLeapYear(year:u32)->bool{
    if year%4!=0{
        false
    }else{
        if year%100==0{
                year%400==0
        }else{
            true
        }
    }       

 
}

pub fn sumOfTwo<T>(a:T,b:T)->T{
    a+b
}
pub fn table(num:u32){
    for i in 1..=10{
        println!("{num}*{i}={num*i}");
    }
}

pub fn is_odd_even()->bool{
    let  mut input=String::new();
    let io=io::stdin();
    io.read_line(&mut input).expect("Could not read input");
    let input=input.parse().expect("Enter a valid number")  ;
    let ans=match input%2{
        0=>true,
        1=>false
    };
    ans




}

pub fn area_of_circle(radius:f64){
    PI*radius*radius
}

pub fn area_of_triangle(a:f64,b:f64,c:f64)->f64{
    let s=(a+b+c)/2 as f64;    
    sqrtf64(s*(s-a)*(s-b)*(s-c))
}

pub fn fibonacci(n:i32)->i32{
    let mut one=1;
    let mut two=1;
    
    let mut ans=one+two;
    for i in 0..n{

        ans=one+two;
        two=one;
        one=ans;
        
    }
    ans

}

pub fn diff(num)


struct Node <T>{
    data:T,
    next:Option<Box<Node<T>>>

}


pub struct LinkedList<T>{

    head:Option<Box<Node<T>>>
}
impl <T> LinkedList<T>{
    pub fn new()->Self{
        LinkedList{head:None}
    }

    pub fn push(&mut self,val:T){
        let node = Box::new(Node {data : val ,next : self.head.take()});


        self.head=Some(node);    
    }

    pub fn pop(&mut self)->Option<T>{
        let val=self.head.take();
        match val{
            Some(mut node)=>{
                self.head=node.next.take();
                Some(node.data)
            },
            None=>None
        }

    }


    pub fn length(&self)->u8{
        let mut len=0;
        let mut curr_node=self.head.as_ref().unwrap();
        while  curr_node.next.is_some(){
                len+=1;
                curr_node=curr_node.next.as_ref();
                }
    len

    
}
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
