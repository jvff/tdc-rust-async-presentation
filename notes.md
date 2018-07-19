# Programação Assíncrona em Rust

## Concorrência

### Tarefas em um programa

- Existem várias formas de organizar um programa
- Uma delas é como sendo composto por várias tarefas
  - Uma tarefa é executada ao ser solicitada por um usuário
  - Outra tarefa é executada periodicamente
  - E outra é executada toda vez que um evento externo ocorre
    - Notificação externa
    - Inserção de um dispositivo de armazenamento

### Relação entre as tarefas

- Com um programa organizado em tarefas
- As tarefas tem relações entre si
- Algumas dependem da execução de outras
- Algumas podem ser executadas em paralelo, terminando em momentos diferentes
- Outras não precisam ou não podem ser executadas se outra tiver executada

### Execução das tarefas

- Como executar as tarefas
- O jeito mais simples uma por uma
- Esse não é o modo mais eficiente
- Diferentes tarefas usam recursos diferentes
- Recursos diferentes tem latência diferentes
  - Processador
  - Memória
  - Disco rígido
  - Rede de comunicação
- Enquanto uma tarefa usa um recurso, outros podem estar ociosos

### Concorrência

- Uma forma de aumentar a eficiência é através do uso de *threads*
- Quando uma operação realiza uma operação com latência alta, outra *thread* é
  executada
- Múltiplas *threads* podem estar aguardando recursos diferentes ao mesmo tempo

### Troca de contexto

- Há um custo para interromper uma *thread* e começar outra
- O sistema operacional não sabe o que a *thread* precisa, então copia tudo
  associado a ela
  - Troca de contexto
  - Registradores do processador
  - Mapeamento de memória
  - Lista de arquivos abertos
- Consequentemente, separar o programa em *threads* aumenta a eficiência
- Mas quanto mais *threads* maior o custo de troca de contexto
  - Menos eficiência
- Uma *thread* por tarefa pode não ser uma boa ideia

### Uso de um número limitado de *threads*

- Uma boa prática é manter um número máximo de *threads* e uma fila de tarefas
- Cada *thread* executa uma tarefa por vez
- Quando uma *thread* termina uma tarefa, ela pega outra da fila
- A dificuldade é em projetar as tarefas
  - Qual o melhor nível de granularidade?

### Programação assíncrona

- Se o programa pudesse controlar a troca de contexto, ela seria menos custosa
- Sabe o que cada tarefa precisa, então só move o essencial
- Precisaria controlar o escalonamento das tarefas
- Saber quais tarefas estão aguardando qual recurso
- O sistema operacional oferece uma ferramenta útil
  - Aguardar o término de uma operação entre várias listadas
  - Dado uma lista de operações em execução
  - O sistema operacional pausa a *thread* atual
  - Só será acordada quando uma ou mais das operações terminarem
  - A *thread* continua e sabe quais operações terminaram
- Essa prática usa um laço de eventos

## Futures

### Introdução

- Uma *crate* para programação assíncrona
- Oferece primitivas para representar a programação assíncrona
  - Tarefa
  - Operação
  - Executador
- Oferece ferramentas para facilitar a representação de uma operação complexa

### Tarefa

- Struct `futures::task::Task` permite acesso a uma tarefa
  - Permite acordar a tarefa, para indicar que uma operação associada a ela está
    pronta
- `futures::task::current()` retorna a `Task` para acessar a tarefa atual

### Executador

- Trait `futures::future::Executor`
  - Conceito abstrato de um executador de tarefas
  - `Executor::execute()` agenda uma operação a ser executada
- Podem haver diversas implementações de executadores
  - Uma única *thread* rodando as operação
  - Um número predeterminado de *threads*

```
pub trait Executor<F: Future<Item = (), Error = ()>> {
    fn execute(&self, future: F) -> Result<(), ExecuteError<F>>;
}
```

### Operação

- Trait `futures::Future`
  - Representa uma operação assíncrona
  - Método `poll` é chamado para tentar avançar a operação
  - Quando terminado, retorna um ítem ou um erro
  - Se não tiver terminado:
    - **Precisa** se preparar para acordar a tarefa quando um recurso necessário
      se tornar disponível
    - Retorna `Async::NotReady` indicando que a tarefa pode pausar

### Implementando operações

- Na maioria dos casos não é preciso implementar um `Future` manualmente
- Mesmo se necessário, geralmente é feito compondo outros `Future`s
  - Basta tomar o cuidade de retornar `Async::NotReady` se e **somente se** pelo
    menos um dos `Future`s usados internamente também tiver retornado
    `Async::NotReady`
    - (ou seja, algum `Future` interno se preparou para acordar a tarefa)

### Compondo uma operação atrás de outra

- `map`: aplica uma função no ítem resultante do `Future`, transformando-o em
  outro tipo
- `map_err`: aplica uma função no erro resultante do `Future`, transformando-o
  em outro
- `and_then`: aplica uma função no ítem resultante do `Future`, e em seguida 
  roda o novo `Future` retornado pela função (somente se o `Future` inicial for
  bem sucedido)
- `or_else`: aplica uma função no erro resultanto do `Future`, e em seguida
  roda o novo `Future` retornado pela função (somente se o `Future` inicial não
  for bem sucedido)
- `then`: aplica uma função no resultado do `Future`, e em seguida roda o
  novo `Future` retornado pela função
- `flatten`: se a operação retornar outra operação em forma de `Future`, este
  método compõe as duas em uma só operação

### Compondo operações simultâneas

- `select`: retorna um `Future` que terminará quando qualquer um dos dois
  `Future`s juntados terminar ou falhar
- `join`: retorna um `Future` que só terminará quando os dois `Future`s juntados
  terminarem ou quando um deles falhar

### Compondo operações simultâneas

- O tipo do erro deve ser o mesmo para as duas operações
  - `map_err` pode ser utilizado para igualar os tipos

### Fluxos

- Representação de operações que produzem vários ítens, um por vez
- Pode falhar, mas isso **não** termina o fluxo
- Versão assíncrona do `Iterator`

```
pub trait Stream {
    type Item;
	type Error;

	fn poll_next(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error>;
}
```

### Compondo operações em fluxos

- Operações em fluxo também podem ser compostas
- `map`, `map_err`, `and_then`, `or_else`, `then`: são análogos mas operam em
  cada resultado do fluxo
- `flatten` agora permite que `Stream`s sejam ítens, e são então encadeados ao
  fluxo de resultados final


### Enviando em fluxo

- Representação de uma operação para enviar algo de forma assíncrona
- O ítem representado agora é o tipo de ítem que pode ser enviado
- `start_send` tenta enfileirar o envio de um ítem
- Mas o ítem só é enviado de fato quando `poll_complete` indicar que a operação
  de enviar todos os ítens enfileirados estiver concluída

```
pub trait Sink {
    type SinkItem;
	type SinkError;

	fn start_send(
		&mut self, 
		item: Self::SinkItem
	) -> Result<(), Self::SinkError>;

	fn poll_complete(&mut self) -> Result<Async<()>, Self::SinkError>;
}
```

### Compondo escritas em fluxo

- Operações de escrita em fluxo também podem ser compostas
- No entanto, agora as operações operam nos ítens antes que eles sejam enviados
  - `map`: altera um ítem antes de enviá-lo
  - `with`: aplica um `Future` em um ítem para alterá-lo
  - `with_flat_map`: usa uma função que transforma cada ítem em um `Stream`, e
    envia os ítens que ela produz
  - `fanout`: duplica os ítens para enviar em mais de um `Sink`
  - `flush`: retorna um `Future` para a operação de enviar os ítens enfileirados
  - `send`: retorna um `Future` para enviar um ítem, e depois retorna o fluxo de
    volta
  - `send_all`: retorna um `Future` para enviar todos os ítens de um `Stream`

### Canais assíncronos

- Uma outra funcionalidade útil em `futures` é a criação de canais de
  comunicação entre tarefas
- Dois tipos:
  - `oneshot` permite enviar um único ítem de uma tarefa a outra
    - O `oneshot::Receiver` implementa `Future` para receber esse ítem
  - `mpsc` permite enviar vários ítens, sendo que várias podem receber mas
    somente uma pode receber
    - O `mpsc::Sender` implementa `Sink`
    - O `mpsc::Receiver` implementa `Stream`
- Muito útil para começar tarefas assíncronas separadas para possibilitar que
  cada uma seja executada em um *thread* separado

## Tokio

### Introdução

- Tokio é uma *crate* complementar à crate `futures`
- Fornece implementações para os tipos abstratos de `futures`
  - Laço de eventos, executadores, e operações assíncronas
  - Comunicação na rede
  - Acesso à arquivos
  - Temporizador
- Função mais importante: `tokio::run`, que rodará um `Future` especificado
- Está em processo de reorganização
  - Existem várias *crates*
    - `tokio-core`
    - `tokio-timer`
    - `tokio-io`
    - `tokio-codec`
  - Existe uma *crate* principal com o objetivo de centralizar as
    funcionalidades comuns

### Executador

- Responsável por avançar `Future`s chamando `poll` quando a tarefa receber um 
  evento
- Por padrão, uma *thread* com um executador é criado para cada núcleo do
  processador
  - `tokio_executor::DefaultExecutor::current()` pode ser usado para se obter um
    acesso ao executador padrão

### Laço de eventos

- Chamado `Reactor`
- Registro de todas as operações assíncronas em andamento
- Responsável por aguardar por eventos e acordar tarefas com base neles
- Pode ser acessado através de um `Handle`, necessário para registrar novas
  operações de baixo nível
  - Exemplos
    - Abrir uma conexão de rede
    - Acessar um arquivo
    - Configurar um temporizador

### Temporizador

- Controla operações de tempo
- `Delay` é um `Future` que termina em um determinado instante
- `Interval` é um `Stream` que produz um ítem por vez a um intervalo de tempo
  determinado

### Limitar tempo de uma operação

- `Deadline` é um `Future` que junto `Delay` com outro `Future`, para garantir
  que a operação termine antes de um determinado instante
  - Seria equivalente a um *timeout*
  - Poderia ser implementado como `future.select(Delay::new(instante))`, mas sem
    precisar lidar com um resultado duplo

### Entrada assíncrona

- `tokio::io` oferece ferramentas para leitura e escrita assíncrona
- `AsyncRead` é o equivalente assíncrono do `Read`
  - `poll_read` é o método usado para ler de forma assíncrona
  - Para implementar `AsyncRead`, basta implementar `Read` e se a operação de
    leitura for bloquear, precisa agendar para a tarefa ser acordada quando a
    operação estiver pronta e retornar `Async::NotReady`

### Saída assíncrona

- `AsyncWrite` é o equivalente assíncrono do `Write`
  - `poll_write` tenta enfileirar dados a serem escritos
  - `poll_flush` tenta enviar dados enfileirados
  - `shutdown` tenta enviar dados enfileirados e depois fechar a escrita
  - Para implementar `AsyncWrite`, basta implementar `Write` de forma análoga ao
    `AsyncRead`
    - `shutdown` precisa ser implementado

### Operações de entrada e saída

- Algumas funções facilitam a composição de operações, retornando `Future`s
  - `copy`: copia todos os dados de um leitor para um escritor
  - `flush`: envia todos os dados enfileirados
  - `read_exact`: lê um número determinado de bytes
  - `read_to_end`: lê todos os bytes de um leitor
  - `write_all`: escreve todos os bytes de um *buffer*
  - `shutdown`: finaliza um escritor
- Uma função retorna um `Stream`
  - `lines`: lê linha por linha em forma de `String`s

### Entrada e saída padrão

- As funções `tokio::io::stdout`, `stdin` e `stderr` permite utilizar a entrada
  e as saídas padrão de forma assíncrona
- Precisa tomar cuidado para usá-los de forma concorrente

### Encodificação

- A *crate* `tokio-codec` auxilia na conversão de leitores e escritores em
  fluxos.
- A *trait* `Encoder` possibilita especificar como transformar bytes lidos em
  ítens de `Stream`

### Decodificação

- A *trait* `Decoder` possibilita especificar como transformar ítens de um
  `Sink` em bytes a serem escritos
  - Somente o método `decode` precisa ser especificado
  - O `decode_eof` é chamado quando não há mais bytes a serem lidos, então
    possibilita gerar um último ítem especial se necessário
- O método `Decoder::framed` ou `AsyncRead::framed` transforma um canal
  assíncrono em um fluxo assíncrono usando a codificação especificada

### Manipulando arquivos

- `tokio::fs` é muito semelhante a `std::fs`, mas é assíncrono
- `File::open` e `File::create` retornam `Future`s que quando executados irão
  realizar suas devidas operações
- `File` implementa `AsyncRead` e `AsyncWrite`, permitindo leitura e escrita de
  forma assíncrona também

```
let escreve_arquivo = tokio::fs::File::create("arquivo.txt")
	.and_then(|arquivo| tokio::io::write_all(arquivo, &dados));
```

### Comunicação na rede

- `tokio::net` oferece ferramentas para comunicação através da rede
- `TcpStream` possibilita comunicação através do protocolo TCP
  - `TcpStream::connect` retorna um `Future` que se resolverá quando a conexão 
    estiver estabelecida
  - `TcpStream` implementa `AsyncRead` e `AsyncWrite
- `TcpListener` pode ser usado para esparar conexões de clientes
  - `TcpListener::incoming` retorna um `Stream` de `TcpStream`s
- `UdpSocket` pode ser usado para comunicação UDP
  - `UdpSocket::send_dgram` retorna um `Future` que enviará um pacote de dados
  - `UdbSocket::recv_dgram` retorna um `Future` que resultará em um pacote de
    dados recebido

## Futuro

### Futures 0.2

- Nova versão já disponível
- Reestruturação para ficar mais compreensível
- Separação para evolução mais rápida dos módulos
- Preparação para `async` e `await`

### Tokio 0.2

- Ainda não disponível
- Funcionalidades novas ainda estão sendo introduzidas na versão 0.1
- Migração para `Futures` 0.2
- Mudança recente de direção
  - Foco nas funcionalidades mais abrangentes
  - `tokio_proto` receberá menos atenção
- Separação para evoluir mais rapidamente

### Contexto evidente

- Agora fica mais claro que um `Future` está diretamento associado ao contexto
  de uma tarefa

```
pub trait Future {
    type Item;
    type Error;
    fn poll(
        &mut self, 
        cx: &mut Context
    ) -> Result<Async<Self::Item>, Self::Error>;
}
```

```
pub trait Stream {
    type Item;
	type Error;

	fn poll_next(
		&mut self, 
		cx: &mut Context
	) -> Result<Async<Option<Self::Item>>, Self::Error>;
}
```

### Tarefas mais claras

- Struct `futures::task::Context`
  - Representa o contexto de uma tarefa
  - `Context::executor() -> &mut Executor` dá acesso ao executador da tarefa
  - `Context::spawn(...)` agenda uma operação no executador
  - `Context::waker() -> &Waker` dá acesso ao mecanismo para acordar a tarefa
- Struct `futures::task::Waker`
  - `Waker::wake()` permite acordar uma determinada tarefa, para indicar que uma
	operação associada a este `Waker` está pronta
  - Substitui `Task::notify()`

### Algumas mudanças de nomes

- `Async::NotReady` -> `Async::Pending`
- `Stream::poll` -> `Stream::poll_next`
- `Sink::poll_complete` -> `Sink::poll_flush`

``` 
pub enum Async<T> {
    Ready(T),
    Pending,
}
```

### Envio em fluxo fica um pouco mais complicado

- Mais métodos
  - `poll_ready` precisa ser chamado antes para verificar se o `Sink` está
    pronto para começar a enviar um ítem
  - `start_send` enfileira o envio de um ítem
  - Mas o ítem só é enviado de fato quando `poll_flush` indicar que a operação
    de enviar todos os ítens enfileirados estiver concluída
  - `poll_close` tenta enviar os ítens restantes e fechar o fluxo

```
pub trait Sink {
    type SinkItem;
	type SinkError;

	fn poll_ready(
		&mut self, 
		cx: &mut Context
	) -> Result<Async<()>, Self::SinkError>;

	fn start_send(
		&mut self, 
		item: Self::SinkItem
	) -> Result<(), Self::SinkError>;

	fn poll_flush(
		&mut self, 
		cx: &mut Context
	) -> Result<Async<()>, Self::SinkError>;

	fn poll_close(
		&mut self, 
		cx: &mut Context
	) -> Result<Async<()>, Self::SinkError>;
}
```

### Operações de leitura e escrita foram movidas

- As representações para operações de leitura assíncrona e escrita assíncrona
  agora estão na *crate* `futures`

```
pub trait AsyncRead {
    fn poll_read(
        &mut self, 
        cx: &mut Context, 
        buf: &mut [u8]
	) -> Result<Async<usize>, Error>;

	unsafe fn initializer(&self) -> Initializer { ... }

	fn poll_vectored_read(
		&mut self, 
		cx: &mut Context, 
		vec: &mut [&mut IoVec]
	) -> Result<Async<usize>, Error> { ... }
}
```

```
pub trait AsyncWrite {
    fn poll_write(
		&mut self, 
		cx: &mut Context, 
		buf: &[u8]
	) -> Result<Async<usize>, Error>;

	fn poll_flush(&mut self, cx: &mut Context) -> Result<Async<()>, Error>;

	fn poll_close(&mut self, cx: &mut Context) -> Result<Async<()>, Error>;

	fn poll_vectored_write(
		&mut self, 
		cx: &mut Context, 
		vec: &[&IoVec]
	) -> Result<Async<usize>, Error> { ... }
}
```

### Separação dos métodos de composição de operações

- A *trait* `Future` ficou mais simples
- Fica mais consolidada, mudando pouco (ou nada) entre versões
- Os métodos de composição foram movidas para `FutureExt`
  - `map`
  - `map_err`
  - `and_then`
  - `or_else`
  - `then`
  - `flatten`
  - etc.
- Novos métodos podem ser adicionados sem alterar o `Future`
- A *trait* só é usada se uma composição for necessária


### Facilitando operações de leitura e escrita

- E foram adicionados métodos genéricos para facilitar o uso das operações de
  leitura e escrita
- Todos transformam o tipo "primitivo" de leitura ou escrita assíncrona em uma
  operação um pouco mais "alto nível" que implementa `Future`

```
pub trait AsyncReadExt: AsyncRead {
    fn copy_into<W>(self, writer: W) -> CopyInto<Self, W>
	where
		W: AsyncWrite,
	{ ... }

	fn read<T>(self, buf: T) -> Read<Self, T>
	where
		T: AsMut<[u8]>,
	{ ... }

	fn read_exact<T>(self, buf: T) -> ReadExact<Self, T>
	where
		T: AsMut<[u8]>,
	{ ... }

	fn read_to_end(self, buf: Vec<u8>) -> ReadToEnd<Self> { ... }

	fn split(self) -> (ReadHalf<Self>, WriteHalf<Self>)
	where
		Self: AsyncWrite,
		{ ... }
}

impl<T> AsyncReadExt for T where
    T: AsyncRead + ?Sized 
{ ... }
```

```
pub trait AsyncWriteExt: AsyncWrite {
    fn flush(self) -> Flush<Self> { ... }

	fn close(self) -> Close<Self> { ... }

	fn write_all<T>(self, buf: T) -> WriteAll<Self, T>
	where
		T: AsRef<[u8]>,
	{ ... }
}

impl<T> AsyncWriteExt for T where
    T: AsyncWrite + ?Sized
{ ... }
```

### Macros

- A grande expectativa para o futuro da programação assíncrona em Rust é a
  funcionalidade de `async/await`
- Um conjunto de mudanças na linguagem
  - Geradores
  - Representação de dados imóveis
  - Macros procedurais
- Resultará nas macros `async` e `await!`, que possibilitam escrever código mais
  simples

```
pub fn enviar_dado(dado: Dado) -> impl Future<Item = (), Error = Erro> {
	TcpStream::connect(ENDERECO_SERVIDOR)
		.map_err(Erro::from)
		.and_then(|tcp_stream| {
			let fluxo  = tcp_stream.framed(Codificacao);
			fluxo.send(dado)
				.map_err(Erro::from)
				.and_then(|fluxo| io::shutdown(fluxo))
			
		})
}
```

```
#[async]
pub fn enviar_dado(dado: Dado) -> Result<(), Erro> {
    let tcp_stream = await!(TcpStream::connect(ENDERECO_SERVIDOR))?;
	let fluxo = tcp_stream.framed(Codificacao);

	let fluxo = await!(fluxo.send(dado))?;
	await!(io::shutdown(fluxo))?;

	Ok(())
}
```
