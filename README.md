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

  * Límits globals
  * Límits per donant
  * Donació mínima
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

1. Instal·la el CLI de **MultiversX**:

```bash
npm install -g @multiversx/sdk-cli
```

2. Compila el contracte:

```bash
erdpy contract build
```

3. Fes deploy al testnet:

```bash
erdpy contract deploy --project . --recall-nonce \
  --pem wallet.pem --gas-limit=20000000 \
  --arguments <target> <deadline>
```

4. Interactua amb els endpoints (`fund`, `claim`, etc.)

---

## 📁 Estructura del projecte

```
/src
  └── contract.rs   # Lògica del smart contract
/tests              # Tests (opcional)
multiversx.json     # Configuració del projecte
README.md
```

---

## 📜 Llicència

Aquest projecte es pot utilitzar lliurement sota llicència MIT (o la que prefereixis).
