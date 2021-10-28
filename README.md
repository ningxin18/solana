

### Instruction-指令

![image-20210906183410597](/Users/pundix041/Library/Application Support/typora-user-images/image-20210906183410597.png)

整体流程是DApp客户端将自定义的指令数据序列化 到data里面，然后将账号信息和data发到链上，Solana节点为其找到要执行的程序，并将账号信息和数据data 传递给合约程序，合约程序里面将这个data数据在反序列化，得到客户端传过来的具体参数。

### Account-账户

Solana链上的信息，记录在文件中，这个文件在Solana上表现为Account, 类似Unix世界里面的：一切皆是文件，所以用户所需要支付的就是一个文件存储所需要的花费，是以SOL计价的。这里衍生出一个概念， 如果想要关闭文件的话，那么只要把这个Account的SOL都转走，那么这个Account对应的地址，在链上就没有钱 来买位置了，也就会被删除掉了。data字段内容的大小，但是从花费硬盘资源的角度，确实比较类似）。 这里的”is_writable”表示文件是否可执行，如果是可执行的，那么就是一个智能合约账号。 而data里面则是文件的内容，类似电脑上的ls 列出的文件属性，和cat列出来的文件内容，这里是二进制的buffer来表示。每个文件都要由一个程序来创建，这个程序称之为这个文件的拥有者，也就是这里的owner。

### 合约结构

### ![img](https://miro.medium.com/max/1400/1*vMUIvfVhT71vIfh_mXgEhA.png)

```
1. 解析由runtime传过来的instruction
2. 执行instruction对应的逻辑
3. 将执行结果中需要落地的部分，pack打包输出到指定的Account文件
```

根据这个逻辑结构，依次创建如下几个文件：

```
instruction.rs ： 解析由runtime传过来的instruction
processor.rs : 针对instruction的合约逻辑
state.rs : 将需要存储的内容进行打包存储
error.rs: 出错处理，定义各种错误
entrypoint.rs : 结合“entrypoint”特性，封装合约入口
```

solana program deploy /Users/pundix041/rust/ningxin18/solana-program-library/target/deploy/helloworld.so
Program Id: 3tpz9jRHR79GM4xMBGpJjqEpYn9NNNEeN38UCG124i5z










