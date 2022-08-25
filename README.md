# binary-converter
## Descrição curta da lógica:
- Converter a parte inteira para binário
- Converter a parte decimal multiplicando-a por 2 até que o resultado seja `exatamente 1.0`.
- Mover a vírgula para a direção do primeiro dígito 1.
    - Se mover para a esquerda, conta-se positivamente.
    - Se mover para a direita, conta-se negativamente.
- Pegar o `expoente` e convertê-lo para `excesso de 127`.
- Para obter a `mantissa`:
    - Se o `expoente` for positivo, juntar os dois binários e começar a percorrer a `string resultante` a partir do `index` igual ao `expoente`
        - Enquanto o tamanho da `mantissa` for menor que `23` adicionar os caracteres um a um na `string resultante`.
        - Se o tamanho da `string resultante` for atingido antes da `mantissa` estar completa:
            - Se o valor da parte decimal do número for `.0`, ir adicionando `0`.
            - Se não  igualar `i` ao `expoente` e recomeçar o ciclo
    - Se o `expoente` for negativo, começar a percorrer o `binário da parte decimal` a partir do `index` igual ao `valor absoluto do expoente`.
        - Se o `binário da parte decimal` estiver vazio, preencher a `mantissa` com `0` até atingir o tamanho 23.
        - Se não, ir adicionando os caracteres do `binário da parte decimal` um a um na `mantissa` até atingir o tamanho.
            - Se o tamanho do `binário da parte decimal` for atingido antes da `mantissa` estar completa, igualar `i` ao `expoente` e recomeçar o ciclo.    
- Para montar o número concatenar as seguintes variáveis nessa ordem:
    - [**{`SINAL`}**|**{`EXPOENTE CONVERTIDO`}**|**{`MANTISSA: REPETE A DÍZIMA ATÉ ACABAR OS BITS`}**]
    
## Motivação
- Esse projeto foi desenvolvido como trabalho da disciplina de `Lógica Digital` do `Curso em Bacharelado em Ciências da Computação da UTFPR-CM`
- O objetivo do trabalho era fazer com que o aluno visse na prática como um número `float` seria armazenado na memória e compreender melhor o processo que está por trás disso.

## Continuidade do projeto
- No futuro tenho interesse em continuar esse projeto adicionando conversores em outras linguagens, como, `C`, `Typescript` e `Rust`.

