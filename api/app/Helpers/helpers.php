<?php

if (! function_exists('make_response')) {
    function make_response(
        $data = null,
        string $message = '',
        $code = 200,
        bool $success = true,
        $errors = null
    ) {
        return response()->json([
            'success' => $success,
            'message' => $message,
            'data'    => $data,
            'errors'  => $errors,
            'code'  => $code,
        ], $code);
    }
}
