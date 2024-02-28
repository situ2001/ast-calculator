"use client";

import Image from "next/image";
import styles from "./page.module.css";
import { useRef, useState } from "react";

export default function Home() {
  let evalFn = useRef<Promise<(s: string) => number>>(
    new Promise((resolve, reject) => {
      import("ast-calculator")
        .then((module) => {
          resolve(module.eval_expr);
        })
        .catch(reject);
    })
  );

  let [result, setResult] = useState<string>("");
  let [resultTime, setResultTime] = useState<number>(0);

  let [apiResult, setApiResult] = useState<string>("");
  let [apiResultTime, setApiResultTime] = useState<number>(0);

  return (
    <main className={styles.main}>
      <form
        action="/eval"
        method="GET"
        onSubmit={(e: React.FormEvent<HTMLFormElement>) => {
          e.preventDefault();

          let expr = e.currentTarget.querySelector("input")!.value;

          if (!expr) {
            setResult("Please provide an expression");
            setApiResult("Please provide an expression");
            return;
          }

          let start = Date.now();
          evalFn.current.then((f) => {
            try {
              let result = f(expr);
              console.log("Result: ", result);
              let end = Date.now();
              setResult(result.toString());
              setResultTime(end - start);
            } catch (e) {
              console.error("Error: ", e);
              setResult("Error");
            }
          });

          fetch(`/eval?expr=${encodeURIComponent(expr)}`).then((res) => {
            res.text().then((text) => {
              let end = Date.now();
              setApiResult(text);
              setApiResultTime(end - start);
            });
          });
        }}
      >
        <input
          type="text"
          name="expr"
          placeholder="Enter an expression"
          className={styles.input}
        />
        <button className={styles.button}>Execute</button>
      </form>
      <div>
        <h1>Local Result ({resultTime} ms)</h1>
        <h2>{result}</h2>
      </div>

      <div>
        <h1>HTTP Result ({apiResultTime} ms)</h1>
        <h2>{apiResult}</h2>
      </div>
    </main>
  );
}
