<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\AuthController;
use App\Http\Controllers\TownController;

Route::get('/hello', [TownController::class, 'hello']);

Route::get('/user', function (Request $request) {
    return $request->user();
})->middleware('auth:sanctum');

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
