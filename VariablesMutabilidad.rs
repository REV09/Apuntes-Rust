fn main(){
/*  En rust para declarar una variable se utiliza la palabra let
    seguido del nombre con el que se le identificara y el valor
    asignado mediante un "=" y su respectivo punto y coma

    Para mostrar en pantalla una variable acompañada de una cadena
    de texto se usan llaves y seguido del texto se coloca una coma
    "," la cual va fuera de las comillas y seguida de esta el 
    nombre de la variable de esta manera:
    println!("Variable x {}",x); 

    
    Las variables en rust por defecto son inmutables
    Si intetas ejecutar este fragmento de codigo: 
    fn main() {
        let x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }
    >>>Mandara un error debido a que no se puede asignar otra vez
        un valor a una variable inmutable<<<
    

    Para ello hay que declarar una variable como mutable con la 
    palabra mut de la siguiente manera: 
    
    let mut [nombre de la variable];
    este codigo es un ejemplo de variables mutables

*/
    let mut num1 = 5;
    println!("La variable num1 vale : {}",num1);
    num1 = 6;
    println!("La variable num1 ahora vale: {}", num1);
}