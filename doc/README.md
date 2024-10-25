# How to make a transaction using the web IDE

The Simfony web IDE can only make a restricted form of transaction: There is 1 transaction input, 1 transaction output and 1 fee output _(Liquid has explicit fee outputs)_. Confidential transactions or assets other than Bitcoin are not supported.

![Screenshot of mempool.space](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/mempool1.png)

## Write the main function

Open [the Simfony web IDE](https://simfony.dev/) and write the main function of your program.

_You can leave the default main function as it is. Customize it if you want._

![Screenshot of the web IDE](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/webide0.png)

## Generate an address

Click the "Address" button to copy the address of your program to the clipboard.

Leave the web IDE tab open. You will need it later.

![Screenshot of the web IDE](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/webide1.png)

## Fund the address

Paste the address into [the Liquid testnet faucet](https://liquidtestnet.com/faucet) and press the "Send assets" button.

![Screenshot of the Liquid testnet faucet](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/faucet1.png)

Copy the ID of the funding transaction to your clipboard.

![Screenshot of the Liquid testnet faucet](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/faucet2.png)

## Look up the funding transaction

Paste the ID of the funding transaction into the [Blockstream Explorer for Liquid testnet](https://blockstream.info/liquidtestnet/).

![Screenshot of the Blockstream Explorer](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/esplora1.png)

Scroll down and find the Simfony UTXO. The Liquid testnet faucet always sends 100000 tL-BTC. In our example, the Simfony UTXO is vout = 1.

![Screenshot of the Blockstream Explorer](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/esplora2.png)

## Enter UTXO data into the web IDE

Enter the ID of the funding transaction and the vout into the web IDE.

_You can leave the remaining fields as they are. Feel free to customize._

![Screenshot of the Simfony web IDE](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/webide2.png)

## Sign the spending transaction

Click the "Sig 0" button to generate a signature for a transaction that spends the Simfony UTXO.

![Screenshot of the Simfony web IDE](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/webide3.png)

Paste the signature into the `mod witness {...}` section.

![Screenshot of the Simfony web IDE](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/webide4.png)

## Generate the spending transaction

Click the "Transaction" button to copy the spending transaction to your clipboard.

![Screenshot of the Simfony web IDE](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/webide5.png)

## Broadcast the spending transaction

Paste the spending transaction into the [Blockstream Liquid testnet explorer](https://blockstream.info/liquidtestnet/tx/push) and click the "Broadcast transaction" button.

![Screenshot of the Simfony web IDE](https://raw.githubusercontent.com/uncomputable/simfony-webide/master/doc/esplora3.png)

If everything worked, the explorer will open the broadcast transaction. In this case, congratulations, you made a Simfony transaction on Liquid testnet!!!

If you see an error message, take a look at the following "Troubleshooting" section.

## Cryptic error message

Cause.

Action to take.

## "Transaction not found" (Blockstream Explorer)

Fake error. The transaction actually worked :)

Wait for 1 minute and reload the page.

## `bad-txns-inputs-missingorspent`

The UTXO doesn't exist.

Double check the txid. You might have to wait for one minute for the UTXO to be included in the blockchain.

## `bad-txns-in-ne-out, value in != value out`

The input value does not equal the output value.

Double-check the UTXO info (vout and value). Check that the fee is lower than the input value.

## `bad-txns-fee-outofrange`

The fee does not cover the transaction weight.

Increase the fee.

## `non-final`

The lock time is higher than the current block height.

Decrease the locktime or wait until the block height is high enough.

## `non-BIP68-final`

The sequence is higher than the current block height plus the UTXO height.

Decrease the sequence or wait until the block height is high enough.

## `dust`

You are creating a dust transaction output.

The fee consumes the entire input value. Decrease the fee.

## `non-mandatory-script-verify-flag (Assertion failed inside jet)`

A Simplicity jet fails.

Double-check the conditions that your Simfony program enforces. Update the witness data or transaction parameters.

Every time you change the transaction parameters, the signature hash of the transaction changes. In this case, you need to **regenerate signatures** using the "Key Store" tab.
