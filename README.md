# Progetto Rust-WASM

Questo progetto contiene diverse funzioni implementate in Rust e compilate in WebAssembly (WASM). Le funzioni coprono vari ambiti, tra cui operazioni su array, manipolazione di immagini, calcoli matematici, operazioni crittografiche, gestione di dati JSON e operazioni su stringhe.

## Tecnologie Utilizzate

- **Rust**: Linguaggio di programmazione utilizzato per implementare le funzioni.
- **WebAssembly (WASM)**: Tecnologia utilizzata per compilare il codice Rust in un formato eseguibile nel browser.
- **wasm-bindgen**: Libreria Rust per facilitare l'interazione tra Rust e JavaScript.
- **Serde**: Libreria Rust per la serializzazione e deserializzazione dei dati.
- **Librerie di compressione e hashing**: Utilizzate per operazioni crittografiche e di compressione.

## Funzionalità Principali

### Moduli

- **array.rs**: Funzioni per l'ordinamento di array.
- **images.rs**: Funzioni per la manipolazione delle immagini.
- **math.rs**: Funzioni matematiche come calcolo della media, somma, fattoriale, massimo e minimo.
- **crypto.rs**: Funzioni per il calcolo di checksum, compressione e decompressione dei dati, e hashing.
- **json.rs**: Funzioni per la gestione dei dati JSON.
- **string.rs**: Funzioni per il calcolo della frequenza delle parole in un testo.

## Compilazione

Per compilare il progetto, eseguire il seguente comando:

```sh
cargo build --target wasm32-unknown-unknown
```

## Importazione nel Progetto

Dopo la compilazione, è possibile importare il modulo WebAssembly nel proprio progetto come un semplice script. Il file compilato si trova nella cartella `pkg`.

Esempio di importazione in JavaScript (`run_scripts.js`):

```javascript
import init, { functionName } from './pkg/your_project_name.js';

async function run() {
    await init();
    // Utilizza le funzioni importate
    functionName();
}

run();
```

## Esempi di Utilizzo

### Esempio di Utilizzo di `calculate_mean` da `math.rs`

```rust
let data = vec![1.0, 2.0, 3.0, 4.0];
let mean = calculate_mean(&data);
assert_eq!(mean, 2.5);
```

### Esempio di Utilizzo di `word_frequency` da `string.rs`

```rust
let input_text = "hello world hello";
let word_freq = word_frequency(input_text);
console.log(word_freq); // Outputs: {hello: 2, world: 1}
```

## Requisiti

- **Rust**: Assicurarsi di avere Rust installato. È possibile installarlo da qui.
- **wasm-pack**: Strumento per compilare i progetti Rust in WebAssembly. Installabile con il comando:
  ```sh
  cargo install wasm-pack
  ```

## Avvio del Progetto

Per avviare il progetto, eseguire il seguente comando:

```sh
wasm-pack build
```

Questo comando compila il progetto e genera i file necessari nella cartella `pkg`.

## Utilizzo con React

La libreria può essere buildata per funzionare in un progetto React utilizzando un contesto (`useContext`). Ecco un esempio di come creare un contesto per caricare e utilizzare il modulo WASM in un'applicazione React:

### Creazione del Contesto

```javascript
import React, { createContext, useState, useEffect, useContext, Suspense } from 'react';
import { loadWasm } from '../wasm-loader.js';

// Step 1: Creazione del Context
const MyContext = createContext();

// Step 2: Creazione del Provider
export const MyProvider = ({ children }) => {
  const [wasm, setWasm] = useState(null);  // Stato per il modulo WASM
  const [isLoading, setIsLoading] = useState(true);  // Stato di caricamento

  useEffect(() => {
    const loadWasmModule = async () => {
      try {
        const wasmModule = await loadWasm('rust_scripts.js');
        setTimeout(() => {
          setIsLoading(false);
          setWasm(wasmModule);
        }, 1000);
      } catch (error) {
        console.error('Errore nel caricamento del WASM:', error);
      } finally {
        setIsLoading(true);  // Setta lo stato a false una volta caricato
      }
    };

    loadWasmModule();  // Avvia il caricamento del WASM
  }, []);  // Carica il WASM una sola volta al montaggio del componente

  // Fornisci il modulo WASM caricato attraverso il Context
  return (
    <MyContext.Provider value={{ wasm, isLoading }}>
      {!isLoading ? children : <>Loading</>}  {/* Non renderizzare la UI finché non è pronto */}
    </MyContext.Provider>
  );
};

export const useWasmContext = () => useContext(MyContext);
```

### Utilizzo del Modulo WASM

Una volta creato il contesto, è possibile utilizzare il modulo WASM nel proprio componente React:

```javascript
import React from 'react';
import { useWasmContext } from './MyProvider';

const MyComponent = () => {
  const { wasm, isLoading } = useWasmContext();

  if (isLoading) {
    return <div>Loading...</div>;
  }

  const resizedBytesWASM = wasm.resize_image(imageBytes, 200, 200);

  return (
    <div>
      {/* Utilizza resizedBytesWASM come necessario */}
    </div>
  );
};

export default MyComponent;
```



# arrays.rs

Questo file contiene due funzioni principali per l'ordinamento di vettori di interi: `quick_sort` e `merge_sort`. Entrambe le funzioni sono implementate in Rust e utilizzano algoritmi di ordinamento noti per la loro efficienza.

## Funzioni

### `quick_sort`

La funzione `quick_sort` implementa l'algoritmo di ordinamento quicksort. Questo algoritmo segue il paradigma divide-et-impera, partizionando ricorsivamente l'array attorno a un elemento pivot e ordinando i sotto-array risultanti.

- **Argomenti**: Un vettore di interi (`Vec<i32>`) da ordinare.
- **Ritorna**: Un nuovo vettore contenente gli interi ordinati in ordine crescente.
- **Complessità temporale**: O(n log n) in media, O(n^2) nel caso peggiore.
- **Complessità spaziale**: O(n) nel caso peggiore a causa della profondità dello stack dalla ricorsione.

#### Esempio di utilizzo

```rust
let unsorted_array = vec![5, 2, 9, 1, 5, 6];
let sorted_array = quick_sort(unsorted_array);
println!("{:?}", sorted_array); // Outputs: [1, 2, 5, 5, 6, 9]
```

### `merge_sort`

La funzione `merge_sort` implementa l'algoritmo di ordinamento MergeSort. Questo algoritmo è noto per il suo comportamento stabile e la complessità temporale prevedibile di O(n log n).

- **Argomenti**: Un vettore mutabile (`Vec<i32>`) contenente la lista di interi da ordinare.
- **Ritorna**: Un vettore ordinato di interi in ordine crescente.
- **Complessità temporale**: O(n log n) in tutti i casi (migliore, peggiore, media).
- **Complessità spaziale**: O(n) a causa dello spazio extra utilizzato per l'array ausiliario.

#### Esempio di utilizzo

```rust
let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
let sorted_arr = merge_sort(arr);
println!("{:?}", sorted_arr); // Outputs: [1, 1, 2, 3, 4, 5, 5, 6, 9]
```
---

# bytes.rs

Questo file contiene due funzioni principali per la manipolazione di dati binari: `count_zero_bits` e `xor_bytes`. Entrambe le funzioni sono implementate in Rust e utilizzano operazioni bitwise per analizzare e trasformare i dati.

## Funzioni

### `count_zero_bits`

La funzione `count_zero_bits` conta il numero totale di bit a zero nei dati forniti. Questa funzione prende una slice di byte (`&[u8]`) e calcola il numero totale di bit a zero in tutti i byte.

- **Argomenti**: Una slice di byte (`&[u8]`) contenente i dati da analizzare.
- **Ritorna**: Il numero totale di bit a zero come `u32`.
- **Complessità temporale**: O(n), dove `n` è il numero di byte nei dati di input.
- **Complessità spaziale**: Minima, proporzionale alla dimensione della slice di input.

#### Esempio di utilizzo

```rust
let input_data = [0b11001100, 0b10101010, 0b11110000];
let zero_bits_count = count_zero_bits(&input_data);
println!("Zero bits count: {}", zero_bits_count); // Outputs the total zero bits in the byte slice
```

### `xor_bytes`

La funzione `xor_bytes` esegue un'operazione XOR bitwise su due slice di byte e restituisce il risultato come un vettore di byte. L'operazione XOR viene eseguita in modo pairwise su ciascun byte delle due slice di input.

- **Argomenti**: 
  - `data1`: La prima slice di byte (`&[u8]`) su cui applicare l'operazione XOR.
  - `data2`: La seconda slice di byte (`&[u8]`) su cui applicare l'operazione XOR.
- **Ritorna**: Un vettore di byte (`Vec<u8>`) che rappresenta il risultato dell'operazione XOR sui byte corrispondenti di `data1` e `data2`.
- **Complessità temporale**: O(n), dove `n` è la lunghezza della slice di input più corta.
- **Complessità spaziale**: Proporzionale alla lunghezza della slice di input più corta.

#### Esempio di utilizzo

```rust
let data1 = [0b11010101, 0b10101010];
let data2 = [0b01100011, 0b11110000];
let result = xor_bytes(&data1, &data2);
println!("{:?}", result); // Outputs: [0b10110110, 0b11011010]
```

---

# crypto.rs

Questo file contiene diverse funzioni per la compressione, decompressione, hashing e calcolo di checksum dei dati. Le funzioni sono implementate in Rust e utilizzano varie librerie per eseguire operazioni efficienti sui dati.

## Funzioni

### `calculate_crc32`

La funzione `calculate_crc32` calcola il checksum CRC32 dei dati forniti. Questo checksum è utilizzato comunemente per il controllo degli errori nella trasmissione e archiviazione dei dati.

- **Argomenti**: Una slice di byte (`&[u8]`) contenente i dati da hashare.
- **Ritorna**: Il checksum CRC32 come `u32`.
- **Complessità temporale**: O(n), dove `n` è la dimensione della slice di input.

#### Esempio di utilizzo

```rust
let data = b"Hello, world!";
let checksum = calculate_crc32(data);
println!("CRC32 checksum: {}", checksum);
```

### `compress_data`

La funzione `compress_data` comprime i dati forniti e pre-pende la loro dimensione originale. Questo permette di decomprimere i dati e recuperare la dimensione originale senza bisogno di contesto esterno.

- **Argomenti**: Una slice di byte (`&[u8]`) contenente i dati da comprimere.
- **Ritorna**: Un vettore di byte (`Vec<u8>`) rappresentante i dati compressi con la dimensione originale pre-posta.
- **Complessità temporale**: Dipende dall'algoritmo di compressione, tipicamente O(n).

#### Esempio di utilizzo

```rust
let data = b"Hello, world!";
let compressed = compress_data(data);
println!("Compressed data: {:?}", compressed);
```

### `decompress_data`

La funzione `decompress_data` decomprime i dati che sono stati compressi con una dimensione pre-posta. Restituisce i dati originali.

- **Argomenti**: Una slice di byte (`&[u8]`) contenente i dati compressi con la dimensione originale pre-posta.
- **Ritorna**: Un vettore di byte (`Vec<u8>`) rappresentante i dati decompressi (originali).
- **Errori**: Panica se il processo di decompressione fallisce.

#### Esempio di utilizzo

```rust
let compressed = compress_data(b"Hello, world!");
let decompressed = decompress_data(&compressed);
assert_eq!(decompressed, b"Hello, world!");
```

### `deflate_compress`

La funzione `deflate_compress` comprime i dati utilizzando l'algoritmo Deflate con un livello di compressione specificato. Il livello di compressione è impostato a 6, che è un compromesso tra velocità e rapporto di compressione.

- **Argomenti**: Una slice di byte (`&[u8]`) contenente i dati da comprimere.
- **Ritorna**: Un vettore di byte (`Vec<u8>`) rappresentante i dati compressi.
- **Livello di compressione**: 6 (può variare da 1 a 9).

#### Esempio di utilizzo

```rust
let data = b"Hello, world!";
let compressed = deflate_compress(data);
println!("Compressed data: {:?}", compressed);
```

### `deflate_decompress`

La funzione `deflate_decompress` decomprime i dati che sono stati compressi utilizzando l'algoritmo Deflate. Restituisce i dati originali.

- **Argomenti**: Una slice di byte (`&[u8]`) contenente i dati compressi.
- **Ritorna**: Un vettore di byte (`Vec<u8>`) rappresentante i dati decompressi (originali).
- **Errori**: Panica se il processo di decompressione fallisce.

#### Esempio di utilizzo

```rust
let compressed = deflate_compress(b"Hello, world!");
let decompressed = deflate_decompress(&compressed);
assert_eq!(decompressed, b"Hello, world!");
```

### `sha256_hash`

La funzione `sha256_hash` calcola l'hash SHA-256 dei dati forniti. SHA-256 è una funzione di hash crittografica che produce un output di dimensione fissa (32 byte) indipendentemente dalla dimensione dell'input.

- **Argomenti**: Una slice di byte (`&[u8]`) contenente i dati da hashare.
- **Ritorna**: Un vettore di 32 byte (`Vec<u8>`) rappresentante l'hash SHA-256 dei dati di input.

#### Esempio di utilizzo

```rust
let input_data = b"Hello, world!";
let hash = sha256_hash(input_data);
println!("SHA-256 hash: {:?}", hash);
```

---

# images.rs

Questo file contiene due funzioni principali per la manipolazione delle immagini: `invert_colors` e `grayscale`. Entrambe le funzioni sono implementate in Rust e utilizzano operazioni efficienti per trasformare i dati delle immagini.

## Funzioni

### `invert_colors`

La funzione `invert_colors` inverte i valori di colore di un array di byte. Ogni byte rappresenta un componente di colore (ad esempio, valori RGB o in scala di grigi) dove 0 è nero e 255 è bianco. L'inversione viene ottenuta sottraendo ogni byte da 255.

- **Argomenti**: Una slice mutabile di byte (`&mut [u8]`) che rappresenta i valori di colore. Dopo l'esecuzione della funzione, ogni byte sarà sostituito con il suo valore invertito (255 - valore originale).
- **Complessità temporale**: O(n), dove `n` è la lunghezza della slice di input.
- **Ottimizzazione**: Utilizza istruzioni SIMD (Single Instruction, Multiple Data) per invertire i colori in blocchi di 16 byte alla volta, migliorando le prestazioni.

#### Esempio di utilizzo

```rust
let mut colors = vec![0, 128, 255, 100, 50];
invert_colors(&mut colors);
assert_eq!(colors, vec![255, 127, 0, 155, 205]);
```

### `grayscale`

La funzione `grayscale` converte un'immagine rappresentata come un array piatto di pixel RGBA in scala di grigi, preservando il canale alpha (trasparenza). Ogni pixel viene convertito in un singolo valore di scala di grigi basato sui canali rosso, verde e blu utilizzando una formula di media ponderata. Il canale alpha rimane invariato.

- **Argomenti**: Una slice di byte (`&[u8]`) che rappresenta un'immagine dove ogni set di 4 byte consecutivi corrisponde a un singolo pixel nel formato `[R, G, B, A]`.
- **Ritorna**: Un `Vec<u8>` che rappresenta i dati dell'immagine in scala di grigi nello stesso formato RGBA.
- **Formula di conversione**: La formula per calcolare il valore di scala di grigi è:
  ```rust
  Gray = (0.299 * R) + (0.587 * G) + (0.114 * B)
  ```

#### Esempio di utilizzo

```rust
let input_image: Vec<u8> = vec![
    255, 0, 0, 255, // Pixel rosso (opaco)
    0, 255, 0, 255, // Pixel verde (opaco)
    0, 0, 255, 255, // Pixel blu (opaco)
    255, 255, 255, 255, // Pixel bianco (opaco)
];
let output_image = grayscale(&input_image);
```

---
## json.rs

### Funzioni

### `parse_csv_to_json`

La funzione `parse_csv_to_json` analizza una stringa CSV e la converte in un vettore di oggetti JSON (`JsValue`). Ogni riga del CSV viene letta e trasformata in un array JSON, con ogni campo della riga che diventa un elemento dell'array.

- **Argomenti**: 
  - `content`: Una stringa contenente i dati CSV da analizzare.
- **Ritorna**: 
  - `Result<Vec<JsValue>, JsValue>`: Un risultato contenente un vettore di oggetti `JsValue` che rappresentano i record CSV analizzati o un errore `JsValue` in caso di fallimento.
- **Complessità temporale**: O(n), dove `n` è il numero totale di campi nel CSV.
- **Utilizzo della memoria**: Cresce con la dimensione del contenuto CSV e il numero di righe e campi.

#### Esempio di utilizzo

```rust
let csv_content = "name,age,city\nJohn,30,New York\nAlice,25,Los Angeles";
let json_result = parse_csv_to_json(csv_content.to_string());
match json_result {
    Ok(json_records) => {
        for record in json_records {
            println!("{}", record.as_string().unwrap());
        }
    }
    Err(e) => {
        console_error!("{}", e.as_string().unwrap());
    }
}
```

### `record_to_json`

La funzione `record_to_json` converte un singolo record CSV in un oggetto JSON (`JsValue`). Ogni campo del record viene convertito in una stringa e poi serializzato come un array JSON.

- **Argomenti**: 
  - `record`: Un riferimento a un `StringRecord` che rappresenta una singola riga dai dati CSV.
- **Ritorna**: 
  - `Result<JsValue, JsValue>`: Un risultato contenente la rappresentazione JSON serializzata del record CSV come `JsValue` o un errore `JsValue` in caso di fallimento.
- **Complessità temporale**: O(m), dove `m` è il numero di campi nel record CSV.
- **Utilizzo della memoria**: Proporzionale al numero di campi nel record.

#### Esempio di utilizzo

```rust
let record = StringRecord::from(vec!["John", "30", "New York"]);
let json_result = record_to_json(&record);
match json_result {
    Ok(json) => println!("{}", json.as_string().unwrap()),
    Err(e) => console_error!("{}", e.as_string().unwrap()),
}
```

## math.rs

### Funzioni

### `calculate_mean`

La funzione `calculate_mean` calcola la media di un array di valori `f64`. Utilizza operazioni SIMD per sommare gli elementi in blocchi di 2 per migliorare le prestazioni.

- **Argomenti**: 
  - `data`: Una slice di numeri `f64` per cui calcolare la media.
- **Ritorna**: 
  - `f64`: La media dei valori in `data`. Se la slice è vuota, restituisce 0.0.
- **Complessità temporale**: O(n), dove `n` è il numero di elementi nella slice.
- **Requisiti**: Supporto SIMD per WebAssembly (`target-feature=+simd128`).

#### Esempio di utilizzo

```rust
let data = vec![1.0, 2.0, 3.0, 4.0];
let mean = calculate_mean(&data);
assert_eq!(mean, 2.5);
```

### `sum`

La funzione `sum` calcola la somma di un vettore di interi `i32`. Utilizza i metodi iterator di Rust per sommare gli elementi in modo efficiente.

- **Argomenti**: 
  - `numbers`: Un `Vec<i32>` contenente gli interi da sommare.
- **Ritorna**: 
  - `i32`: La somma degli interi nel vettore di input.
- **Complessità temporale**: O(n), dove `n` è il numero di elementi nel vettore.

#### Esempio di utilizzo

```rust
let numbers = vec![1, 2, 3, 4, 5];
let result = sum(numbers);
assert_eq!(result, 15);
```

### `factorial`

La funzione `factorial` calcola il fattoriale di un intero non negativo `n` utilizzando i metodi range e iterator di Rust.

- **Argomenti**: 
  - `n`: Un intero non negativo (`u32`) per cui calcolare il fattoriale.
- **Ritorna**: 
  - `u32`: Il fattoriale dell'intero di input `n`.
- **Complessità temporale**: O(n), dove `n` è il valore di input.

#### Esempio di utilizzo

```rust
let result = factorial(5);
assert_eq!(result, 120); // 5! = 120
```

### `max`

La funzione `max` restituisce il valore massimo in una slice di interi `i32`. Se la slice è vuota, restituisce -1.

- **Argomenti**: 
  - `arr`: Una slice di interi `i32`.
- **Ritorna**: 
  - `i32`: Il valore massimo nella slice o -1 se la slice è vuota.

#### Esempio di utilizzo

```rust
let arr = vec![1, 2, 3, 4, 5];
let max_value = max(&arr);
assert_eq!(max_value, 5);
```

### `min`

La funzione `min` restituisce il valore minimo in una slice di interi `i32`. Se la slice è vuota, restituisce -1.

- **Argomenti**: 
  - `arr`: Una slice di interi `i32`.
- **Ritorna**: 
  - `i32`: Il valore minimo nella slice o -1 se la slice è vuota.

#### Esempio di utilizzo

```rust
let arr = vec![1, 2, 3, 4, 5];
let min_value = min(&arr);
assert_eq!(min_value, 1);
```

## string.rs

### Funzioni

### `word_frequency`

La funzione `word_frequency` calcola la frequenza di ogni parola in un testo fornito e restituisce il risultato come un oggetto JavaScript (`JsValue`). La funzione divide il testo in parole utilizzando gli spazi bianchi come delimitatori e conta quante volte ogni parola appare.

- **Argomenti**: 
  - `text`: Una slice di stringa (`&str`) contenente il testo di input per cui calcolare le frequenze delle parole.
- **Ritorna**: 
  - `JsValue`: Un oggetto JavaScript che rappresenta le frequenze delle parole. Ogni chiave nell'oggetto è una parola (stringa) e ogni valore è la frequenza (intero) di quella parola nel testo.
- **Complessità temporale**: O(n), dove `n` è il numero di parole nel testo di input.

#### Esempio di utilizzo

```rust
let input_text = "hello world hello";
let word_freq = word_frequency(input_text);
console.log(word_freq); // Outputs: {hello: 2, world: 1}
```


## Contributi

I contributi sono benvenuti! Sentiti libero di aprire issue e pull request per migliorare il progetto.

## Licenza

Questo progetto è distribuito sotto la licenza MIT. Vedi il file `LICENSE` per maggiori dettagli.