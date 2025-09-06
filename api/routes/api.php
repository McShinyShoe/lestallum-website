<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\AuthController;
use App\Http\Controllers\TownController;
use App\Http\Controllers\TaskController;

Route::get('/hello', [TownController::class, 'hello']);

Route::get('/user', function (Request $request) {
    return $request->user();
})->middleware('auth:sanctum');

// Task Routes
Route::prefix('task')->group(function () {
    Route::get('/{id}', [TaskController::class, 'show']);
    Route::get('/{id}/files/{filename}', [TaskController::class, 'showFile']);
    Route::middleware('auth:sanctum')->group(function () {
        Route::get('/', [TaskController::class, 'index']);
        Route::post('/{id}/join', [TaskController::class, 'join']);
        Route::post('/{id}/leave', [TaskController::class, 'leave']);
        Route::middleware('role:admin,mayor,comayor,assistant')->group(function () {
            Route::post('/{id}', [TaskController::class, 'store']);
            Route::put('/{id}', [TaskController::class, 'update']);
            Route::delete('/{id}', [TaskController::class, 'destroy']);
        });
    });
});

// Authentication Routes
Route::post('/login', [AuthController::class, 'login']);
Route::middleware('auth:sanctum')->group(function () {
    Route::post('/logout', [AuthController::class, 'logout']);
    Route::get('/verify', [AuthController::class, 'generateVerificationCode']);
    Route::post('/verify', [AuthController::class, 'submitVerificationCode']);
});

// Town Routes
Route::middleware('auth:sanctum')->group(function () {
    Route::get('/town', [TownController::class, 'get']);
    Route::middleware('role:admin,mayor,comayor')->group(function () {
        Route::put('/town', [TownController::class, 'update']);
    });
});
