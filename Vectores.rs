/*              =====VECTORES=====
    Los vectores son otro tipo de datos compuestos
    que existen en rust, este tambien es una
    coleccion de elementos. La diferencia con la
    tupla es que en este caso todos los elementos
    deben ser del mismo tipo, por lo que no sera 
    necesario declarar el tipo de cada uno de los
    elementos. En un arreglo a diferenci de la tupla
    se deben usar corchetes para declarar un vector
    "[elementos, del, vector]" Ejemplo:
    let vector = [1,2,3,4,5,6];

    Este mismo ejemplo nos sirve para hacer un vector
    de cadenas, numeros, decimales o caracteres.
    Tambien se puede hacer un vector de vectores, lo
    que se le conoce comunmente como una matriz,
    eso ya lo veremos mas adelante.

    En un vector tambien se puede declarar el tipo de
    los datos igual que en una tupla con un par de cambios
    Ejemplo:
    let vector: [i32;4] = [2,4,6,8];

    En este ejemplo el vector tendra datos de tipo entero
    con el tamaño de 32 bits, eso es lo que significa el
    i32, mientras que el punto y coma seguido del numero cuatro
    (;4) significa el numero de datos que tendra de este tipo
    el vector

    Tambien se puede inicializar un vector en el que cada uno de
    sus elementos tendra ya un valor definido de manera inicial
    de dos maneras. La tradicional seria asi:
    let vector = [1,1,1,1,1];

    O podemos hacerlo a la manera de rust de esta forma:
    let vector = [1;5];

    Ambas maneras son igual de validas, en ambos ejemplos se
    inicializara un vector de 5 elementos y cada uno de ellos
    tendra el valor 1 de forma inicial

    Ahora para acceder y almacenar un valor del vector de manera
    individual en una variable es de manera similar que en
    una tupla pero con un cambio:
    let valor1 = vector[posicion del valor]

    En este caso en lugar de usar un ".posicion" se usan los
    corchetes y esto es por que el valor dentro de los
    corchetes rust lo lee como el index del arreglo.

    Cabe mencionar que si se intenta realizar alguna operacion
    en una posicion fuera de un arreglo rust no lo permitira
    y directamente mandara un error.  
*/

fn main(){
    let vector1 = [1,2,3,4,5,6];
    let vector2: [f32;3] = [8.7,6.35,7.7];
    let vector3 = [1,1,1];
    let valor1 = vector3[2];
    let vector4 = [1;3];
    println!("Posicion 2 del vector1: {}",vector1[1]);
    println!("Posicion 3 del vector2: {}",vector2[2]);
    println!("Valor de la variable valor1: {}",valor1);
    println!("Posicion 2 del vector vector4: {}", vector4[1]);
}