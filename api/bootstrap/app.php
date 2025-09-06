<?php

use Illuminate\Foundation\Application;
use Illuminate\Foundation\Configuration\Middleware;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Auth\AuthenticationException;

use App\Http\Middleware\RoleMiddleware;
use App\Http\Middleware\VerifiedMiddleware;
use Illuminate\Auth\Access\AuthorizationException;
use Illuminate\Database\Eloquent\MassAssignmentException;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\QueryException;
use Illuminate\Http\Client\ConnectionException;
use Illuminate\Http\Client\RequestException;
use Illuminate\Validation\ValidationException;
use Laravel\Sanctum\Exceptions\MissingAbilityException;

return Application::configure(basePath: dirname(__DIR__))
    ->withRouting(
        web: __DIR__.'/../routes/web.php',
        api: __DIR__.'/../routes/api.php',
        commands: __DIR__.'/../routes/console.php',
        health: '/up',
    )
    ->withMiddleware(function (Middleware $middleware): void {
        $middleware->alias([
            'role' => RoleMiddleware::class,
            'verified' => VerifiedMiddleware::class,
        ]);
    })
    ->withExceptions(function ($exceptions) {
        $exceptions->render(function (AuthenticationException $e, Request $request) {
            $message = $e->getMessage();
            return make_response(null, $message ?: 'Unauthenticated.', Response::HTTP_UNAUTHORIZED, false, null);
        });
        $exceptions->render(function (AuthorizationException $e, Request $request) {
            $message = $e->getMessage();
            return make_response(null, $message ?: 'Forbidden.', Response::HTTP_FORBIDDEN, false, null);
        });
        $exceptions->render(function (QueryException $e, Request $request) {
            return make_response(null, 'Database query error: ' . $e->getMessage(), Response::HTTP_INTERNAL_SERVER_ERROR, false, null);
        });
        $exceptions->render(function (ValidationException $e, Request $request) {
            $message = $e->getMessage();
            return make_response(null, $message ?: 'Validation failed.', Response::HTTP_UNPROCESSABLE_ENTITY, false, $e->errors());
        });
        $exceptions->render(function (MassAssignmentException $e, Request $request) {
            return make_response(null, 'Mass assignment error: ' . $e->getMessage(), Response::HTTP_INTERNAL_SERVER_ERROR, false, null);
        });
        $exceptions->render(function (ModelNotFoundException $e, Request $request) {
            $message = $e->getMessage();
            return make_response(null, $message ?: 'Resource not found.', Response::HTTP_NOT_FOUND, false, null);
        });
        $exceptions->render(function (RequestException $e, Request $request) {
            return make_response(null, 'External service error: ' . $e->getMessage(), Response::HTTP_BAD_GATEWAY, false, null);
        });
        $exceptions->render(function (ConnectionException $e, Request $request) {
            $message = $e->getMessage();
            return make_response(null, $message ?: 'Failed to connect to external service.', Response::HTTP_BAD_GATEWAY, false, null);
        });
        $exceptions->render(function (MissingAbilityException $e, Request $request) {
            $message = $e->getMessage();
            return make_response(null, $message ?: 'Token lacks required ability', Response::HTTP_FORBIDDEN, false, null);
        });

        $exceptions->render(function (Throwable $e, Request $request) {
            return make_response(null, 'Server Error: ' . $e->getMessage(), Response::HTTP_INTERNAL_SERVER_ERROR, false, null);
        });
    })
    ->create();
