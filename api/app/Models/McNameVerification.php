<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class McNameVerification extends Model
{
    protected $fillable = ['mc_name', 'code', 'expires_at'];

    protected $casts = [
        'expires_at' => 'datetime',
    ];
}
