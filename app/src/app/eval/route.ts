import { eval_expr } from "ast-calculator";

export const dynamic = 'force-dynamic' // defaults to auto
export async function GET(request: Request) {
  let param = new URL(request.url).searchParams.get('expr');

  // 500 error
  if (!param) {
    return new Response('Please provide an expression', { status: 500 });
  }

  let result;
  try {
    result = eval_expr(param).toString();
    console.log('Expr received:', param, 'Result:', result);
    return new Response(result, { status: 200 });
  } catch (e) {
    console.error('Error:', e);
    return new Response('Error', { status: 500 });
  }
}
