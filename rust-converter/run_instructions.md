# Instruções para executar o código

## Requisitos:
* Ter o [rust](https://www.rust-lang.org/pt-BR) instalado na máquina
	* Realize a instalação segundo as instruções para o seu SO


## Executando:
* Pelo terminal:
	* Vá até o local do projeto e entre no diretório `rust-converter`:
        * Use o comando `cargo run` para executar o projeto
        ```zsh
        cd <path>/binary-converter/rust-converter
        cargo run
        ```
    
* Informe um número `float` qualquer, como aqueles contidos em `my_test_data.md`.
* Espere o resultado da conversão e o compare com o resultado do site do [conversor da IEEE](https://www.h-schmidt.net/FloatConverter/IEEE754.html).
* Eles devem ser iguais
* **Exemplo de resultado esperado**
    ```zsh
    ############################################################
    # Binary Converter - RUST
	Type any float number:
    130.56


	Stored 130.56:
    [01000011000000101000111101011100] - 32 bits
    ############################################################
    ```

## Adendos:
* **`0.0` e `-0.0` tem o mesmo retorno neste meu conversor.**
* **Se encontrar algum problema grave e puder/quiser relatar no [repositório do projeto](https://github.com/afmireski/binary-converter/issues) agradeço**
