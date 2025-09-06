<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Hash;
use App\Models\User;

class AuthController extends Controller
{
    public function login(Request $request)
    {
        $request->validate([
            'mc_name' => 'required|string',
            'password' => 'required|string',
        ]);

        $user = User::where('mc_name', $request->mc_name)->first();

        if (!$user || !Hash::check($request->password, $user->password)) {
            return make_response(null, 'Invalid credentials', Response::HTTP_UNAUTHORIZED, false);
        }

        $token = $user->createToken('api-token')->plainTextToken;

        return make_response(['token' => $token], 'Login successful');
    }

    public function logout(Request $request)
    {
        $request->user()->currentAccessToken()->delete();
        return make_response(null, 'Token revoked');
    }
}
