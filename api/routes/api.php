<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\TownController;
use Illuminate\Support\Facades\Hash;

use App\Models\User;

Route::get('/hello', [TownController::class, 'hello']);

Route::get('/user', function (Request $request) {
    return $request->user();
})->middleware('auth:sanctum');

// Authentication Routes
Route::post('/login', function (Request $request) {
    $user = User::where('mc_name', $request->mc_name)->first();
    if (!$user || !Hash::check($request->password, $user->password)) {
        return make_response(null, 'Invalid credentials', 401, false);
    }
    return make_response(['token' => $user->createToken('api-token')->plainTextToken], 'Login successful');
});
Route::post('/logout', function (Request $request) {
    $request->user()->currentAccessToken()->delete();
    return make_response(null, 'Token revoked');
})->middleware('auth:sanctum');

// Town Routes
Route::middleware('auth:sanctum')->group(function () {
    Route::get('/town', [TownController::class, 'get']);
    Route::middleware('role:admin,mayor,comayor')->group(function () {
        Route::put('/town', [TownController::class, 'update']);
    });
});
