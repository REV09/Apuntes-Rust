//para colocar un comentario en rust se debe poner doble diagonal asi : //


/*Para colocar comentarios en multiples lineas se coloca una diagonal seguida
de un asterisco como aqui, para cerrar el comentario se coloca un asterisco y
seguido se coloca una diagonal.


Rust es un lenguaje compilado por lo que para ejecutar un archivo se debe de 
compilar primer con el comando "rustc [nombre del archivo.rs]"


Al compilar a diferencia de C++ donde se tiene que colocar otro comando para
nombrar el archivo compilado en caso de compilar en linux, rust genera un
ejecutable con el mismo nombre del archivo compilado, este ejecutable igual
que otros lenguajes compilados solo se podra ejecutar en el SO en el que se 
compilo en este caso el ejecutable de este codigo solo se ejecutara en linux.

Para crear un ejecutable para el SO que deseas hay que compilar en dicho SO,
por lo que tendras que tener instalado rust.

Para ejecutar simplemente usaras el comando ./[nombre del archivo] en el caso
de linux o MacOS
Para ejecutarlo en Windows deberas usar el comando .\[nombre del archivo.exe]

Las instrucciones en rust se deberan acompañar de un ";" al igual que java o
c++

*/
fn main(){
    println!("Hola mundo en Rust");
}
