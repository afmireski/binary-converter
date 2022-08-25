# Instruções para executar o código

## Requisitos:
    * Ter o [python](https://www.python.org) instalado na máquina
        * Realize a instalação segundo as instruções para o seu SO


## Executando:
    * Pelo terminal:
        * Vá até o local do projeto e entre no diretório `python-converter`:
        * Use o comando `python` seguido do nome do arquivo para executá-lo
    ```zsh
        cd <path>/binary-converter/python-converter
        python ./binary-converter.py
    ```

    * Por uma IDE:
        * Abra a pasta `python-converter` na IDE de sua preferência, como o `PyCharm`.
        * Defina que quer executar o arquivo: `binary-converter.py`
    
    * Independente do caminho o programa vai executar.
    * Informe um número `float` qualquer, como aqueles contidos em `run_instructions.md`.
    * Espere o resultado da conversão e o compare com o resultado do site do [conversor da IEEE](https://www.h-schmidt.net/FloatConverter/IEEE754.html).
        * Eles devem ser iguais
    * Resultado esperado
    ```zsh
    ############################################################
    # Binary Converter - PYTHON
	    Type any float number:
	    > 130.56


	    Stored 130.56:
	    > [01000011000000101000111101011100] - 32 bits
    ############################################################
    ```

## Adendos:
    * **`0.0` e `-0.0` tem o mesmo retorno neste meu conversor.**
    * **Se encontrar algum problema grave e puder/quiser relatar no [repositório do projeto](https://github.com/afmireski/binary-converter/issues) agradeço**
