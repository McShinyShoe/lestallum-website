<?php

use Illuminate\Http\Request;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\TownController;

use App\Models\User;

Route::get('/hello', [TownController::class, 'hello']);

Route::get('/user', function (Request $request) {
    return $request->user();
})->middleware('auth:sanctum');

Route::post('/login', function (Request $request) {
    $user = User::where('email', $request->email)->first();
    if (!$user || !\Hash::check($request->password, $user->password)) {
        return make_response(null, 'Invalid credentials', false);
    }
    return make_response(['token' => $user->createToken('api-token')->plainTextToken], 'Login successful');
});

Route::middleware('auth:sanctum')->group(function () {
    Route::middleware('auth:sanctum')->post('/logout', function (Request $request) {
        $request->user()->currentAccessToken()->delete();
        return make_response(null, 'Token revoked');
    });

    Route::get('/town', [TownController::class, 'get']);
    Route::middleware('role:admin')->group(function () {
        Route::put('/town', [TownController::class, 'update']);
    });
});



