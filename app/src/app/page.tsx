"use client";

import { useRef, useState } from "react";

export default function Home() {
  let evalExpressionFunction = useRef<Promise<(s: string) => number>>(
    new Promise((resolve, reject) => {
      import("ast-calculator")
        .then((module) => {
          resolve(module.evalExpression);
        })
        .catch(reject);
    })
  );

  let [result, setResult] = useState<string>("");
  let [resultTime, setResultTime] = useState<number>(0);

  let [apiResult, setApiResult] = useState<string>("");
  let [apiResultTime, setApiResultTime] = useState<number>(0);

  return (
    <main>
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
          evalExpressionFunction.current.then((f) => {
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
        <input type="text" name="expr" placeholder="Enter an expression" />
        <button>Execute</button>
      </form>
      <div>
        <span>Local Result ({resultTime} ms): </span>
        <span>{result}</span>
      </div>

      <div>
        <span>HTTP Result ({apiResultTime} ms): </span>
        <span>{apiResult}</span>
      </div>
    </main>
  );
}
