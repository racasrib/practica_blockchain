# practica_blockchain
Lliurament de la pràctica del curs de Blockchain a l'FP de l'ICE de la URV

---

# 🧱 Crowdfunding Smart Contract (MultiversX)

Aquest repositori conté un Smart Contract escrit en Rust i basat en el framework **MultiversX Smart Contracts**.
El contracte implementa un sistema de **micromecenatge (crowdfunding)** amb control total sobre límits, dates i retorns automàtics.

---

## ✨ Funcionalitats principals

* **Iniciar campanyes** amb:

  * Objectiu de fons (`target`)
  * Data límit de recaptació (`deadline`)
* **Aportacions en EGLD** mitjançant l'endpoint `fund`
* **Límits configurables**:

  * Límits globals (`limit`)
  * Límits per donant (`limit_per_donor`)
  * Donació mínima (`minimum_per_donation`)
* **Sistema d'estats automàtic**:

  * `FundingPeriod`
  * `Successful`
  * `Failed`
* **Mecanismes de claim**:

  * Si la campanya té èxit, només l’owner pot retirar els fons.
  * Si falla, cada donant pot recuperar la seva aportació.

---

## 📦 Arquitectura del Contracte

### **Estat de la Campanya (enum `Status`)**

```rust
FundingPeriod
Successful
Failed
```

### **Variables principals**

* `target`: Quantitat d’EGLD a assolir.
* `deadline`: Timestamp límit.
* `deposit(donor)`: Registre individual de donacions.
* `limit`: Límits globals.
* `limit_per_donor`: Límits per usuari.
* `minimum_per_donation`: Donació mínima.

---

## 🔧 Endpoints

### **init(target, deadline)**

Inicialitza la campanya.
Requisits:

* `target > 0`
* `deadline` ha de ser futur

---

### **set_limit(limit)**

Només per a l’owner.

### **set_limit_per_donor(limit_per_donor)**

Només per a l’owner.

### **set_minimum_per_donation(min_value)**

Només per a l’owner.

---

### **fund() – [payable EGLD]**

Permet aportar fons mentre:

* No s’ha superat el `deadline`
* No se superen els límits globals o per donant
* La donació no és inferior al mínim permès

---

### **claim()**

Comportament segons estat:

* **FundingPeriod:** bloquejat
* **Successful:** l’owner retira els fons
* **Failed:** cada donant recupera el seu `deposit`

---

### **status()**

Retorna l’estat de la campanya segons:

* Temps actual vs deadline
* Fons actuals vs objectiu

---

## 🔍 Views

| View                      | Descripció                             |
| ------------------------- | -------------------------------------- |
| `getCurrentFunds`         | Retorna el balanç actual del contracte |
| `getTarget`               | Objectiu configurat                    |
| `getDeadline`             | Data límit                             |
| `getDeposit(donor)`       | Donació del donant                     |
| `getLimit`                | Límits globals                         |
| `getLimit_per_donor`      | Límits per donant                      |
| `getMinimum_per_donation` | Donació mínima                         |

---

## ▶️ Execució i Testing

Perfecte — aquí tens la **mateixa documentació**, igual de senzilla, però **ben ordenada, clara i agradable de llegir**, sense afegir complexitat extra:

---

## ▶️ Execució i Testing

### **Fem el deployment**

```bash
mxpy contract deploy \
  --bytecode crowdfunding.wasm \
  --proxy=https://devnet-gateway.multiversx.com \
  --arguments 10000000000000000000 1764547200 \
  --gas-limit 15000000 \
  --pem=wallet.pem \
  --send
```

**Owner:**
`erd1kx5rh2usj47t4a87nhqgkm0mvczj9jemw2l0jjtdtqnexwxv03msdr4dld`

**Adreça del contracte:**
`erd1qqqqqqqqqqqqqpgq2klz9lhmzn6v7y535myzwxg5nq4calx203msdkwsjf`

**Consulta del deadline:**

```bash
mxpy contract query erd1qqqqqqqqqqqqqpgq2klz9lhmzn6v7y535myzwxg5nq4calx203msdkwsjf \
  --function getDeadline \
  --proxy https://devnet-api.multiversx.com
```

Resultat:

```
"6938a9f0"
```

---

### **Afegim i verifiquem els límits**

**Límit global:**

```bash
mxpy contract call <contract> --pem=wallet.pem --proxy=https://devnet-gateway.multiversx.com \
  --function set_limit --arguments 20000000000000000000 --gas-limit 15000000 --send
```

Consulta:

```bash
mxpy contract query <contract> --function getLimit --proxy https://devnet-api.multiversx.com
```

Resultat:

```
"01158e460913d00000"
```

---

**Límit per donant:**

```bash
mxpy contract call <contract> --pem=wallet.pem --proxy=https://devnet-gateway.multiversx.com \
  --function set_limit_per_donor --arguments 1000000000000000000 --gas-limit 15000000 --send
```

Consulta:

```bash
mxpy contract query <contract> --function getLimit_per_donor --proxy https://devnet-api.multiversx.com
```

Resultat:

```
"0de0b6b3a7640000"
```

---

**Donació mínima:**

```bash
mxpy contract call <contract> --pem=wallet.pem --proxy=https://devnet-gateway.multiversx.com \
  --function set_minimum_per_donation --arguments 100000000000000000 --gas-limit 15000000 --send
```

Consulta:

```bash
mxpy contract query <contract> --function getMinimum_per_donation --proxy https://devnet-api.multiversx.com
```

Resultat:

```
"016345785d8a0000"
```

---

### **Podem fer donatius i verificar errors**

Intentem donar menys del mínim:

```bash
mxpy contract call <contract> --pem=wallet.pem --function fund \
  --value 10000000000000000 --gas-limit 15000000 --send
```

Resultat:

```
Error -> Cannot accept donations below the minimum contribution limit
```

---

Donació correcta:

```bash
mxpy contract call <contract> --pem=wallet.pem --function fund \
  --value 200000000000000000 --gas-limit 15000000 --send
```

Resultat:

```
Ok
```

---

Intentem superar el límit per donant:

```bash
mxpy contract call <contract> --pem=wallet.pem --function fund \
  --value 3000000000000000000 --gas-limit 15000000 --send
```

Resultat:

```
Error -> Cannot exceed the maximum contribution limit per donor
```

---

## 📜 Llicència

Aquest projecte es pot utilitzar lliurement sota llicència MIT (o la que prefereixis).
