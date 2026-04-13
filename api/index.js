const express = require('express');
const StellarSdk = require('@stellar/stellar-sdk');
require('dotenv').config();

const app = express();
app.use(express.json());

const server = new StellarSdk.SorobanRpc.Server('https://soroban-testnet.stellar.org');
const contract = new StellarSdk.Contract(process.env.CONTRACT_ID);

const buildTransaction = async (operation) => {
    const account = await server.getAccount(process.env.PUBLIC_KEY);
    const tx = new StellarSdk.TransactionBuilder(account, {
        fee: StellarSdk.BASE_FEE,
        networkPassphrase: StellarSdk.Networks.TESTNET
    })
        .addOperation(operation)
        .setTimeout(30)
        .build();
    return tx;
};

const sendTransaction = async (tx) => {
    const preparedTx = await server.prepareTransaction(tx);
    preparedTx.sign(StellarSdk.Keypair.fromSecret(process.env.SECRET_KEY));
    return await server.sendTransaction(preparedTx);
};

app.get('/contador', async (req, res) => {
    try {
        const tx = await buildTransaction(contract.call('get'));
        const sim = await server.simulateTransaction(tx);
        const value = StellarSdk.scValToNative(sim.result?.retval);
        res.json({ count: value });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.post('/contador/increment', async (req, res) => {
    try {
        const tx = await buildTransaction(contract.call('increment'));
        const sim = await server.simulateTransaction(tx);
        const value = StellarSdk.scValToNative(sim.result?.retval);
        res.json({ count: value });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.post('/contador/decrement', async (req, res) => {
    try {
        const tx = await buildTransaction(contract.call('decrement'));
        const sim = await server.simulateTransaction(tx);
        const value = StellarSdk.scValToNative(sim.result?.retval);
        res.json({ count: value });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.post('/contador/reset', async (req, res) => {
    try {
        const tx = await buildTransaction(contract.call('reset'));
        const sim = await server.simulateTransaction(tx);
        res.json({ success: true });
    } catch (error) {
        res.status(500).json({ error: error.message });
    }
});

app.listen(3000, () => console.log('API corriendo en puerto 3000'));
